//! E2E tests for the packaged MCP server binary.
//!
//! These tests launch the compiled `okc` binary with the same stdio transport
//! shape used by OpenCode, then exercise the MCP protocol over a real child
//! process instead of the in-process server implementation.

#![allow(clippy::expect_used, clippy::panic)]

use anyhow::Context;
use rmcp::{
    model::{CallToolRequestParams, CallToolResult, ClientInfo},
    service::{QuitReason, RunningService},
    transport::{
        child_process::TokioChildProcess,
        streamable_http_client::StreamableHttpClientTransportConfig, StreamableHttpClientTransport,
    },
    ClientHandler, RoleClient, ServiceExt,
};
use serde_json::{json, Value};
use std::{
    path::{Path, PathBuf},
    process::Stdio,
    time::{Duration, Instant},
};
use tempfile::TempDir;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
    time::timeout,
};

type Client = RunningService<RoleClient, TestClientHandler>;

/// Dummy client handler for testing.
#[derive(Debug, Clone, Default)]
struct TestClientHandler;

impl ClientHandler for TestClientHandler {
    fn get_info(&self) -> ClientInfo {
        ClientInfo::default()
    }
}

struct StdioSession {
    client: Client,
    stderr_task: tokio::task::JoinHandle<anyhow::Result<String>>,
}

fn packaged_okc_binary() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_okc"))
}

/// Test fixture for the simple repository.
fn setup_simple_repo() -> TempDir {
    let temp_dir = TempDir::new().expect("create temp dir for simple repo");
    let source = Path::new("tests/fixtures/simple");
    copy_dir_all(source, temp_dir.path()).expect("copy simple fixture");
    temp_dir
}

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dst.join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), dst.join(entry.file_name()))?;
        }
    }
    Ok(())
}

async fn collect_child_stderr(stderr: tokio::process::ChildStderr) -> anyhow::Result<String> {
    let mut reader = BufReader::new(stderr).lines();
    let mut output = String::new();

    while let Some(line) = reader.next_line().await? {
        output.push_str(&line);
        output.push('\n');
    }

    Ok(output)
}

async fn launch_packaged_stdio_session(workspace: &TempDir) -> anyhow::Result<StdioSession> {
    let mut command = Command::new(packaged_okc_binary());
    command.current_dir(workspace.path());
    command.args(["serve", "--transport", "stdio"]);

    let (transport, stderr) = TokioChildProcess::builder(command)
        .stderr(Stdio::piped())
        .spawn()
        .context("spawn packaged okc serve --transport stdio")?;

    let stderr = stderr.context("packaged okc stderr was not piped")?;
    let stderr_task = tokio::spawn(async move { collect_child_stderr(stderr).await });
    let client = TestClientHandler
        .serve(transport)
        .await
        .context("initialize packaged MCP session")?;

    Ok(StdioSession {
        client,
        stderr_task,
    })
}

async fn scan_workspace_via_mcp(client: &Client, workspace: &TempDir) -> anyhow::Result<Value> {
    let result = call_tool(
        client,
        "scan",
        Some(json!({
            "roots": [workspace.path().to_string_lossy()],
        })),
    )
    .await
    .context("scan workspace through packaged MCP binary")?;

    let total_files = result["total_files"]
        .as_u64()
        .context("scan response missing total_files")?;
    assert!(
        total_files > 0,
        "scan should discover fixture files in the workspace, got {total_files}"
    );
    assert!(
        workspace.path().join("okc_index.db").exists(),
        "scan should create the default database file in the workspace current directory"
    );

    Ok(result)
}

async fn invoke_tool(
    client: &Client,
    tool_name: &str,
    arguments: Option<Value>,
) -> anyhow::Result<CallToolResult> {
    let mut params = CallToolRequestParams::new(tool_name.to_string());
    if let Some(args) = arguments {
        params = params.with_arguments(args.as_object().expect("args should be object").clone());
    }

    Ok(client.call_tool(params).await?)
}

/// Call a tool and return its structured MCP response.
async fn call_tool(
    client: &Client,
    tool_name: &str,
    arguments: Option<Value>,
) -> anyhow::Result<Value> {
    let result = invoke_tool(client, tool_name, arguments).await?;

    let structured = result
        .structured_content
        .clone()
        .with_context(|| format!("{tool_name} response missing structuredContent"))?;

    let text = result
        .content
        .first()
        .and_then(|c| c.as_text())
        .map(|t| t.text.as_str())
        .expect("Expected text content");

    serde_json::from_str::<Value>(text)
        .with_context(|| format!("{tool_name} text fallback is not valid JSON"))?;

    Ok(structured)
}

/// Call a tool and parse its compatibility text response.
async fn call_tool_text(
    client: &Client,
    tool_name: &str,
    arguments: Option<Value>,
) -> anyhow::Result<Value> {
    let result = invoke_tool(client, tool_name, arguments).await?;
    let text = result
        .content
        .first()
        .and_then(|content| content.as_text())
        .map(|content| content.text.as_str())
        .with_context(|| format!("{tool_name} response missing text fallback"))?;

    Ok(serde_json::from_str(text)?)
}

async fn close_stdio_session(mut session: StdioSession) -> anyhow::Result<(QuitReason, String)> {
    let quit_reason = timeout(Duration::from_secs(5), session.client.close())
        .await
        .context("timed out closing packaged MCP session")??;

    let stderr = timeout(Duration::from_secs(5), session.stderr_task)
        .await
        .context("timed out waiting for packaged okc stderr")??;

    Ok((quit_reason, stderr?))
}

struct HttpSession {
    client: Client,
    stderr_task: tokio::task::JoinHandle<anyhow::Result<String>>,
    child: tokio::process::Child,
}

/// Discover the `http://host:port/mcp` endpoint the packaged server bound to.
///
/// The server binds to an ephemeral port (we ask for port 0) and logs the real
/// address with `tracing::info!("MCP HTTP server listening on ...")`. tracing's
/// `fmt::layer()` writes to stdout (not stderr), so we read the marker from
/// stdout line-by-line until it appears, then keep draining stderr separately.
async fn launch_packaged_http_session(workspace: &TempDir) -> anyhow::Result<HttpSession> {
    let mut command = Command::new(packaged_okc_binary());
    command.current_dir(workspace.path());
    command
        .args([
            "serve",
            "--transport",
            "http",
            "--host",
            "127.0.0.1",
            "--port",
            "0",
        ])
        .env("RUST_LOG", "okc=info")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = command
        .spawn()
        .context("spawn packaged okc serve --transport http")?;
    let stdout = child
        .stdout
        .take()
        .context("packaged okc stdout was not piped")?;
    let mut reader = BufReader::new(stdout).lines();

    let deadline = Instant::now() + Duration::from_secs(15);
    let endpoint = loop {
        if Instant::now() >= deadline {
            anyhow::bail!("timed out waiting for MCP HTTP server to report its endpoint");
        }
        let line = match timeout(Duration::from_secs(15), reader.next_line()).await {
            Ok(Ok(Some(line))) => line,
            Ok(Ok(None)) => anyhow::bail!("packaged okc exited before advertising its endpoint"),
            Ok(Err(error)) => return Err(error).context("read packaged okc stdout"),
            Err(_) => continue,
        };
        if let Some(idx) = line.find("MCP HTTP server listening on ") {
            break line[idx + "MCP HTTP server listening on ".len()..]
                .trim()
                .to_string();
        }
    };

    let stderr = child
        .stderr
        .take()
        .context("packaged okc stderr was not piped")?;
    let stderr_task = tokio::spawn(async move { collect_child_stderr(stderr).await });

    let transport = StreamableHttpClientTransport::from_config(
        StreamableHttpClientTransportConfig::with_uri(endpoint),
    );
    let client = TestClientHandler
        .serve(transport)
        .await
        .context("initialize packaged MCP HTTP session")?;

    Ok(HttpSession {
        child,
        client,
        stderr_task,
    })
}

/// Close the MCP HTTP session: close the client transport and terminate the
/// packaged server (it would otherwise keep serving until killed).
async fn close_http_session(mut session: HttpSession) -> anyhow::Result<(QuitReason, String)> {
    let quit_reason = timeout(Duration::from_secs(5), session.client.close())
        .await
        .context("timed out closing packaged MCP HTTP session")??;

    let _ = session.child.kill().await;
    let _ = session.child.wait().await;

    let stderr = timeout(Duration::from_secs(5), session.stderr_task)
        .await
        .context("timed out waiting for packaged okc stderr")??;

    Ok((quit_reason, stderr?))
}

#[tokio::test]
async fn test_mcp_http_transport_packaged_binary() -> anyhow::Result<()> {
    let repo = setup_simple_repo();
    let session = launch_packaged_http_session(&repo).await?;

    scan_workspace_via_mcp(&session.client, &repo).await?;

    let result = call_tool(
        &session.client,
        "browse",
        Some(json!({
            "path": "",
            "depth": 1,
            "limit": 100
        })),
    )
    .await
    .context("browse workspace root over HTTP transport")?;

    assert!(
        result["directories"].is_array(),
        "browse should return directories over HTTP transport"
    );
    assert!(
        result["directories"]
            .as_array()
            .is_some_and(|dirs| dirs.iter().any(|d| d.as_str() == Some("metrics"))),
        "should have metrics dir over HTTP transport"
    );

    let result = call_tool(
        &session.client,
        "get_document",
        Some(json!({
            "path": "metrics/monthly-revenue.md",
            "include": ["metadata"],
            "max_chars": 12000
        })),
    )
    .await
    .context("fetch document over HTTP transport")?;
    assert_eq!(result["path"], "metrics/monthly-revenue.md");
    assert_eq!(result["concept_type"], "Metric");

    let (quit_reason, stderr) = close_http_session(session).await?;
    assert!(
        matches!(quit_reason, QuitReason::Closed | QuitReason::Cancelled),
        "expected packaged MCP HTTP session to close cleanly, got {quit_reason:?}"
    );
    assert!(
        !stderr.contains("Connection closed"),
        "stderr should not contain a premature connection close: {stderr}"
    );
    Ok(())
}

#[tokio::test]
async fn test_mcp_stdio_transport_all_tools_packaged_binary() -> anyhow::Result<()> {
    let repo = setup_simple_repo();
    let session = launch_packaged_stdio_session(&repo).await?;

    scan_workspace_via_mcp(&session.client, &repo).await?;

    let tools = session
        .client
        .peer()
        .list_all_tools()
        .await
        .context("list tools through packaged MCP binary")?;
    for (expected, required_output_property) in [
        ("scan", "total_files"),
        ("browse", "path"),
        ("get_document", "path"),
        ("get_section", "section"),
        ("search", "results"),
        ("query_metadata", "results"),
        ("get_links", "links"),
        ("get_backlinks", "links"),
        ("traverse", "nodes"),
        ("get_stats", "document_count"),
        ("validate", "summary"),
    ] {
        let tool = tools
            .iter()
            .find(|tool| tool.name == expected)
            .unwrap_or_else(|| panic!("expected tool {expected} to be advertised"));
        let output_schema = tool
            .output_schema
            .as_ref()
            .unwrap_or_else(|| panic!("expected tool {expected} to advertise an outputSchema"));
        assert_eq!(
            output_schema.get("type"),
            Some(&json!("object")),
            "expected tool {expected} outputSchema to have an object root"
        );
        assert!(
            output_schema
                .get("properties")
                .and_then(Value::as_object)
                .is_some_and(|properties| properties.contains_key(required_output_property)),
            "expected tool {expected} outputSchema to describe {required_output_property}"
        );
    }

    let result = call_tool(
        &session.client,
        "browse",
        Some(json!({
            "path": "",
            "depth": 1,
            "limit": 100
        })),
    )
    .await
    .context("browse workspace root through packaged MCP binary")?;

    assert!(
        result["directories"].is_array(),
        "browse should return directories"
    );
    let dirs = result["directories"]
        .as_array()
        .expect("directories should be array");
    assert!(
        dirs.iter().any(|d| d.as_str() == Some("metrics")),
        "should have metrics dir"
    );
    assert!(
        dirs.iter().any(|d| d.as_str() == Some("datasets")),
        "should have datasets dir"
    );

    let result = call_tool(
        &session.client,
        "get_document",
        Some(json!({
            "path": "metrics/monthly-revenue.md",
            "include": ["metadata", "headings"],
            "max_chars": 12000
        })),
    )
    .await
    .context("fetch document through packaged MCP binary")?;

    assert_eq!(result["path"], "metrics/monthly-revenue.md");
    assert_eq!(result["concept_type"], "Metric");
    assert!(result["headings"].is_array());
    assert!(result.get("custom").is_none());
    assert!(result.get("content_hash").is_none());
    assert!(result.get("parent_path").is_none());
    assert!(result.get("links").is_none());
    assert!(result.get("backlinks").is_none());

    for include in [
        "custom",
        "content_hash",
        "parent_path",
        "links",
        "backlinks",
    ] {
        let optional = call_tool(
            &session.client,
            "get_document",
            Some(json!({
                "path": "metrics/monthly-revenue.md",
                "include": [include],
                "max_chars": 12000
            })),
        )
        .await
        .with_context(|| format!("fetch document include '{include}' through packaged MCP"))?;
        assert!(
            optional.get(include).is_some(),
            "requested include '{include}' should be present"
        );
    }

    let enriched = call_tool(
        &session.client,
        "get_document",
        Some(json!({
            "path": "metrics/monthly-revenue.md",
            "include": [
                "metadata",
                "custom",
                "content_hash",
                "parent_path",
                "links",
                "backlinks"
            ],
            "max_chars": 12000
        })),
    )
    .await
    .context("fetch enriched document through packaged MCP binary")?;
    assert_eq!(enriched["custom"]["owner"], "Finance Analytics");
    assert!(enriched["content_hash"]
        .as_str()
        .is_some_and(|hash| !hash.is_empty()));
    assert_eq!(enriched["parent_path"], "metrics");
    assert!(enriched["links"]
        .as_array()
        .is_some_and(|links| !links.is_empty()));
    assert!(enriched["backlinks"].as_array().is_some_and(|links| {
        links
            .iter()
            .any(|link| link["source_path"] == "metrics/churn-rate.md")
    }));

    let invalid_include = invoke_tool(
        &session.client,
        "get_document",
        Some(json!({
            "path": "metrics/monthly-revenue.md",
            "include": ["unknown"]
        })),
    )
    .await
    .context("validate get_document include through packaged MCP binary")?;
    assert_eq!(invalid_include.is_error, Some(true));
    assert!(invalid_include.content.iter().any(|content| content
        .as_text()
        .is_some_and(|text| text.text.contains("Unknown include value"))));

    let result = call_tool(
        &session.client,
        "get_section",
        Some(json!({
            "path": "metrics/monthly-revenue.md",
            "heading": "Definition",
            "max_chars": 5000
        })),
    )
    .await
    .context("fetch section through packaged MCP binary")?;

    assert_eq!(result["section"]["heading"], "Definition");
    assert!(result["section"]["content"]
        .as_str()
        .expect("content should be a string")
        .contains("Monthly Revenue represents"));

    let legacy_section = call_tool_text(
        &session.client,
        "get_section",
        Some(json!({
            "path": "metrics/monthly-revenue.md",
            "heading": "Definition",
            "max_chars": 5000
        })),
    )
    .await?;
    assert_eq!(legacy_section["heading"], "Definition");

    let result = call_tool(
        &session.client,
        "search",
        Some(json!({
            "query": "revenue",
            "limit": 10
        })),
    )
    .await
    .context("search through packaged MCP binary")?;

    assert!(result["results"].is_array());
    assert!(result["total_matches"].is_number());

    let typo_search = call_tool(
        &session.client,
        "search",
        Some(json!({
            "query": "montly reveneu",
            "path_prefix": "metrics/",
            "types": ["Metric"],
            "tags": ["finance"],
            "limit": 5
        })),
    )
    .await
    .context("bounded typo search through packaged MCP binary")?;
    assert!(typo_search["results"].as_array().is_some_and(|results| {
        results
            .iter()
            .any(|result| result["path"] == "metrics/monthly-revenue.md")
    }));

    let filtered_search = call_tool(
        &session.client,
        "search",
        Some(json!({
            "query": "customer",
            "path_prefix": "metrics/",
            "types": ["Metric"],
            "tags": ["customer"],
            "limit": 1
        })),
    )
    .await
    .context("combined filtered search through packaged MCP binary")?;
    assert_eq!(filtered_search["total_matches"], 2);
    assert_eq!(filtered_search["truncated"], true);
    assert_eq!(filtered_search["results"].as_array().map(Vec::len), Some(1));
    assert_eq!(
        filtered_search["results"][0]["path"],
        "metrics/customer-count.md"
    );

    let empty_search = call_tool(
        &session.client,
        "search",
        Some(json!({"query": "quantum entanglement", "limit": 10})),
    )
    .await
    .context("empty search through packaged MCP binary")?;
    assert_eq!(empty_search["total_matches"], 0);
    assert_eq!(empty_search["truncated"], false);
    assert_eq!(empty_search["results"], json!([]));

    let result = call_tool(
        &session.client,
        "query_metadata",
        Some(json!({
            "filter": [
                "type=Metric",
                "tags_contains=finance",
                "path_prefix=metrics/",
                "parse_status=ok",
                "owner=Finance Analytics"
            ],
            "select": ["path", "tags", "owner"],
            "limit": 2
        })),
    )
    .await
    .context("query metadata through packaged MCP binary")?;

    assert_eq!(result["total_matches"], 3);
    assert_eq!(result["truncated"], true);
    assert_eq!(result["results"].as_array().map(Vec::len), Some(2));
    assert_eq!(result["results"][0]["path"], "metrics/churn-rate.md");
    assert_eq!(result["results"][0]["owner"], "Finance Analytics");
    assert_eq!(
        result["results"][0]["tags"],
        json!(["customer", "finance", "retention"])
    );

    let empty = call_tool(
        &session.client,
        "query_metadata",
        Some(json!({
            "filter": ["path_prefix=does-not-exist/"],
            "select": ["path"],
            "limit": 10
        })),
    )
    .await
    .context("query empty metadata result through packaged MCP binary")?;
    assert_eq!(empty["total_matches"], 0);
    assert_eq!(empty["truncated"], false);
    assert_eq!(empty["results"], json!([]));

    let invalid = invoke_tool(
        &session.client,
        "query_metadata",
        Some(json!({"filter": ["type"], "select": ["path"]})),
    )
    .await
    .context("query invalid metadata filter through packaged MCP binary")?;
    assert_eq!(invalid.is_error, Some(true));
    assert!(invalid.content.iter().any(|content| content
        .as_text()
        .is_some_and(|text| text.text.contains("expected key=value"))));

    let result = call_tool(
        &session.client,
        "get_links",
        Some(json!({
            "path": "metrics/monthly-revenue.md"
        })),
    )
    .await
    .context("get links through packaged MCP binary")?;

    assert!(result["links"].is_array());

    let legacy_links = call_tool_text(
        &session.client,
        "get_links",
        Some(json!({
            "path": "metrics/monthly-revenue.md"
        })),
    )
    .await?;
    assert!(legacy_links.is_array());

    let result = call_tool(
        &session.client,
        "get_backlinks",
        Some(json!({
            "path": "metrics/monthly-revenue.md",
            "limit": 50
        })),
    )
    .await
    .context("get backlinks through packaged MCP binary")?;

    assert!(result["links"].is_array());

    let legacy_backlinks = call_tool_text(
        &session.client,
        "get_backlinks",
        Some(json!({
            "path": "metrics/monthly-revenue.md",
            "limit": 50
        })),
    )
    .await?;
    assert!(legacy_backlinks.is_array());

    let result = call_tool(
        &session.client,
        "traverse",
        Some(json!({
            "start": "metrics/monthly-revenue.md",
            "relations": ["links_to", "linked_from"],
            "max_depth": 2,
            "max_nodes": 50
        })),
    )
    .await
    .context("traverse through packaged MCP binary")?;

    assert!(result["nodes"].is_array());
    assert!(result["edges"].is_array());

    let result = call_tool(&session.client, "get_stats", None)
        .await
        .context("get stats through packaged MCP binary")?;

    assert!(result["document_count"].is_number());
    assert!(result["link_count"].is_number());
    assert!(result["heading_count"].is_number());

    let result = call_tool(&session.client, "validate", None)
        .await
        .context("validate through packaged MCP binary")?;

    assert!(result["summary"].is_object());
    assert!(result["issues"].is_array());

    let (quit_reason, stderr) = close_stdio_session(session).await?;
    assert!(
        matches!(quit_reason, QuitReason::Closed | QuitReason::Cancelled),
        "expected packaged MCP session to close cleanly, got {quit_reason:?}"
    );
    assert!(
        !stderr.contains("Connection closed"),
        "stderr should not contain a premature connection close: {stderr}"
    );
    Ok(())
}

#[tokio::test]
async fn test_mcp_enriched_document_respects_configured_response_limit() -> anyhow::Result<()> {
    let repo = setup_simple_repo();
    std::fs::write(repo.path().join("okc.toml"), "max_response_chars = 900\n")?;
    let session = launch_packaged_stdio_session(&repo).await?;
    scan_workspace_via_mcp(&session.client, &repo).await?;

    let result = invoke_tool(
        &session.client,
        "get_document",
        Some(json!({
            "path": "metrics/monthly-revenue.md",
            "include": [
                "body",
                "headings",
                "metadata",
                "custom",
                "content_hash",
                "parent_path",
                "links",
                "backlinks"
            ],
            "max_chars": 12000
        })),
    )
    .await?;
    let structured = result
        .structured_content
        .context("bounded document missing structured content")?;
    assert_eq!(structured["truncated"], true);
    assert!(serde_json::to_string(&structured)?.chars().count() <= 900);

    let (quit_reason, _stderr) = close_stdio_session(session).await?;
    assert!(matches!(
        quit_reason,
        QuitReason::Closed | QuitReason::Cancelled
    ));
    Ok(())
}

/// Test error responses for missing document.
#[tokio::test]
async fn test_mcp_error_missing_document() -> anyhow::Result<()> {
    let repo = setup_simple_repo();
    let session = launch_packaged_stdio_session(&repo).await?;
    scan_workspace_via_mcp(&session.client, &repo).await?;

    let result = session
        .client
        .call_tool(
            CallToolRequestParams::new("get_document".to_string()).with_arguments(
                json!({
                    "path": "nonexistent.md",
                    "include": ["metadata"],
                    "max_chars": 12000
                })
                .as_object()
                .expect("json should be object")
                .clone(),
            ),
        )
        .await?;

    let text = result
        .content
        .first()
        .and_then(|c| c.as_text())
        .map(|t| t.text.as_str())
        .expect("Expected text content");

    assert!(
        text.contains("Error") || text.contains("error") || text == "null",
        "Should return error for missing document, got: {}",
        text
    );

    let (quit_reason, _stderr) = close_stdio_session(session).await?;
    assert!(
        matches!(quit_reason, QuitReason::Closed | QuitReason::Cancelled),
        "expected packaged MCP session to close cleanly, got {quit_reason:?}"
    );
    Ok(())
}

/// Test error responses for invalid path traversal.
#[tokio::test]
async fn test_mcp_error_invalid_path() -> anyhow::Result<()> {
    let repo = setup_simple_repo();
    let session = launch_packaged_stdio_session(&repo).await?;
    scan_workspace_via_mcp(&session.client, &repo).await?;

    let result = session
        .client
        .call_tool(
            CallToolRequestParams::new("get_links".to_string()).with_arguments(
                json!({
                    "path": "invalid/../../../etc/passwd"
                })
                .as_object()
                .expect("json should be object")
                .clone(),
            ),
        )
        .await?;

    let text = result
        .content
        .first()
        .and_then(|c| c.as_text())
        .map(|t| t.text.as_str())
        .expect("Expected text content");

    assert!(
        text.contains("Error") || text.contains("error") || text == "[]",
        "Should handle invalid paths safely, got: {}",
        text
    );

    let (quit_reason, _stderr) = close_stdio_session(session).await?;
    assert!(
        matches!(quit_reason, QuitReason::Closed | QuitReason::Cancelled),
        "expected packaged MCP session to close cleanly, got {quit_reason:?}"
    );
    Ok(())
}

/// Test all tools have at least one e2e call through the packaged binary.
#[tokio::test]
async fn test_all_mcp_tools_covered() -> anyhow::Result<()> {
    let repo = setup_simple_repo();
    let session = launch_packaged_stdio_session(&repo).await?;
    scan_workspace_via_mcp(&session.client, &repo).await?;

    let tools = vec![
        (
            "scan",
            Some(json!({"roots": [repo.path().to_string_lossy()]})),
        ),
        (
            "browse",
            Some(json!({"path": "", "depth": 1, "limit": 100})),
        ),
        (
            "get_document",
            Some(
                json!({"path": "metrics/monthly-revenue.md", "include": ["metadata"], "max_chars": 12000}),
            ),
        ),
        (
            "get_section",
            Some(
                json!({"path": "metrics/monthly-revenue.md", "heading": "Definition", "max_chars": 5000}),
            ),
        ),
        ("search", Some(json!({"query": "revenue", "limit": 10}))),
        (
            "query_metadata",
            Some(json!({"filter": ["type=Metric"], "select": ["path"], "limit": 100})),
        ),
        (
            "get_links",
            Some(json!({"path": "metrics/monthly-revenue.md"})),
        ),
        (
            "get_backlinks",
            Some(json!({"path": "metrics/monthly-revenue.md", "limit": 50})),
        ),
        (
            "traverse",
            Some(
                json!({"start": "metrics/monthly-revenue.md", "relations": ["links_to"], "max_depth": 2, "max_nodes": 50}),
            ),
        ),
        ("get_stats", None),
        ("validate", None),
    ];

    for (tool_name, args) in tools {
        let result = call_tool(&session.client, tool_name, args).await?;
        assert!(
            result.is_object(),
            "Tool {} should return an object matching its outputSchema",
            tool_name
        );
    }

    let (quit_reason, _stderr) = close_stdio_session(session).await?;
    assert!(
        matches!(quit_reason, QuitReason::Closed | QuitReason::Cancelled),
        "expected packaged MCP session to close cleanly, got {quit_reason:?}"
    );
    Ok(())
}

/// Test the packaged binary reports invalid root paths clearly.
#[tokio::test]
async fn test_mcp_stdio_packaged_binary_reports_invalid_root() -> anyhow::Result<()> {
    let workspace = setup_simple_repo();
    let mut command = Command::new(packaged_okc_binary());
    command.current_dir(workspace.path()).args([
        "serve",
        "--transport",
        "stdio",
        "--root",
        "/definitely/does/not/exist",
    ]);

    let output = timeout(Duration::from_secs(10), command.output())
        .await
        .context("timed out waiting for invalid-root CLI smoke test")??;

    assert!(
        !output.status.success(),
        "packaged okc should fail fast for an invalid root"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Root directory does not exist")
            || stderr.contains("ValidationError")
            || stderr.contains("does not exist"),
        "expected a configuration/root error in stderr, got: {stderr}"
    );

    Ok(())
}

#[tokio::test]
async fn test_mcp_scan_rejects_invalid_configuration_before_storage() -> anyhow::Result<()> {
    let workspace = setup_simple_repo();
    let session = launch_packaged_stdio_session(&workspace).await?;
    let db_path = workspace.path().join("invalid-scan.db");
    let missing_root = workspace.path().join("does-not-exist");

    let result = invoke_tool(
        &session.client,
        "scan",
        Some(json!({
            "roots": [missing_root],
            "db_path": db_path,
        })),
    )
    .await
    .context("invoke scan with invalid configuration")?;

    assert_eq!(result.is_error, Some(true));
    assert!(result.content.iter().any(|content| content
        .as_text()
        .is_some_and(|text| text.text.contains("Root directory does not exist"))));
    assert!(
        !db_path.exists(),
        "invalid MCP scan must not create its database"
    );

    close_stdio_session(session).await?;
    Ok(())
}

/// get_document with a near-miss path raises a not-found error whose text suggests the closest existing document.
#[tokio::test]
async fn test_mcp_get_document_typo_suggests_path() -> anyhow::Result<()> {
    let repo = setup_simple_repo();
    let session = launch_packaged_stdio_session(&repo).await?;
    scan_workspace_via_mcp(&session.client, &repo).await?;

    let result = invoke_tool(
        &session.client,
        "get_document",
        Some(json!({
            "path": "metrics/monthly-revenu.md",
            "include": ["metadata"],
            "max_chars": 12000
        })),
    )
    .await
    .context("invoke get_document with a typo path")?;

    assert_eq!(result.is_error, Some(true));
    let text = result
        .content
        .first()
        .and_then(|content| content.as_text())
        .map(|content| content.text.as_str())
        .expect("error response should carry text fallback");
    assert!(
        text.contains("Not found: document"),
        "error should be a structured not-found, got: {text}"
    );
    assert!(
        text.contains("Did you mean:") && text.contains("metrics/monthly-revenue.md"),
        "error should suggest the closest existing path, got: {text}"
    );

    close_stdio_session(session).await?;
    Ok(())
}

/// get_document typo below the bounded candidate window yields no misleading "Did you mean:" hint.
#[tokio::test]
async fn test_mcp_get_document_typo_far_away_has_no_suggestion() -> anyhow::Result<()> {
    let repo = setup_simple_repo();
    let session = launch_packaged_stdio_session(&repo).await?;
    scan_workspace_via_mcp(&session.client, &repo).await?;

    let result = invoke_tool(
        &session.client,
        "get_document",
        Some(json!({
            "path": "finance/budgets/quarterly-review.md",
            "include": ["metadata"],
            "max_chars": 12000
        })),
    )
    .await
    .context("invoke get_document with an unrelated path")?;

    assert_eq!(result.is_error, Some(true));
    let text = result
        .content
        .first()
        .and_then(|content| content.as_text())
        .map(|content| content.text.as_str())
        .expect("error should carry text fallback");
    assert!(
        text.contains("Not found: document"),
        "error should be a structured not-found, got: {text}"
    );
    assert!(
        !text.contains("Did you mean:"),
        "unrelated path must not produce recovery hints (AC #5), got: {text}"
    );

    close_stdio_session(session).await?;
    Ok(())
}

/// get_section on a missing document raises an error with a path suggestion, distinct from a missing heading.
#[tokio::test]
async fn test_mcp_get_section_missing_document_suggests_path() -> anyhow::Result<()> {
    let repo = setup_simple_repo();
    let session = launch_packaged_stdio_session(&repo).await?;
    scan_workspace_via_mcp(&session.client, &repo).await?;

    let missing_doc = invoke_tool(
        &session.client,
        "get_section",
        Some(json!({
            "path": "metrics/monthly-revenu.md",
            "heading": "Definition",
            "max_chars": 5000
        })),
    )
    .await
    .context("invoke get_section on a missing document")?;

    assert_eq!(missing_doc.is_error, Some(true));
    let doc_error = missing_doc
        .content
        .first()
        .and_then(|content| content.as_text())
        .map(|content| content.text.as_str())
        .expect("error should carry text fallback");
    assert!(
        doc_error.contains("Did you mean:") && doc_error.contains("metrics/monthly-revenue.md"),
        "missing document should suggest an existing path, got: {doc_error}"
    );

    close_stdio_session(session).await?;
    Ok(())
}

/// get_section on an existing document with an unknown heading is a base success (`section: null`),
/// not a missing-document error (AC #5).
#[tokio::test]
async fn test_mcp_get_section_unknown_heading_is_not_missing_document() -> anyhow::Result<()> {
    let repo = setup_simple_repo();
    let session = launch_packaged_stdio_session(&repo).await?;
    scan_workspace_via_mcp(&session.client, &repo).await?;

    let result = invoke_tool(
        &session.client,
        "get_section",
        Some(json!({
            "path": "metrics/monthly-revenue.md",
            "heading": "NonexistentHeading",
            "max_chars": 5000
        })),
    )
    .await
    .context("invoke get_section with an unknown heading")?;

    assert_eq!(
        result.is_error,
        Some(false),
        "unknown heading is not an error"
    );
    let text = result
        .content
        .first()
        .and_then(|content| content.as_text())
        .map(|content| content.text.as_str())
        .expect("response should carry text fallback");
    assert_eq!(
        text, "null",
        "section None should serialize to the base null form, got: {text}"
    );

    close_stdio_session(session).await?;
    Ok(())
}
