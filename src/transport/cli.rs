//! Command-line interface for the Open Knowledge Catalog.
//!
//! Defines the `okc` CLI with subcommands for all operations:
//! - `scan` - Index repository
//! - `browse` - Directory browsing
//! - `get` - Document retrieval
//! - `section` - Section extraction
//! - `search` - Full-text search
//! - `metadata` - Structured metadata queries
//! - `links` / `backlinks` - Link navigation
//! - `traverse` - Graph traversal
//! - `validate` - Index validation
//! - `stats` - Index statistics
//! - `serve` - Start MCP server
//! - `watch` - File system watching

use std::net::SocketAddr;
use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

/// Transport type for MCP server.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum TransportType {
    /// Standard input/output transport (for Claude Code, local CLI)
    Stdio,
    /// HTTP/SSE transport (for web clients, remote access)
    Http,
}

/// Top-level CLI structure with all subcommands.
#[derive(Parser)]
#[command(
    name = "okc",
    version,
    about = "Open Knowledge Catalog - index and query markdown knowledge bases"
)]
pub struct Cli {
    /// Path to configuration file (TOML)
    #[arg(long, global = true)]
    pub config: Option<PathBuf>,

    /// The subcommand to execute
    #[command(subcommand)]
    pub command: Command,
}

/// Available CLI commands.
#[derive(Subcommand)]
pub enum Command {
    /// Scan and index the repository.
    Scan {
        /// Root directories to scan
        #[arg(short, long)]
        root: Vec<PathBuf>,
    },
    /// Browse a directory in the knowledge base.
    Browse {
        /// Path to browse (default: root)
        path: Option<String>,
        /// Recursion depth
        #[arg(short, long, default_value = "1")]
        depth: usize,
        /// Maximum results
        #[arg(short, long, default_value = "100")]
        limit: usize,
    },
    /// Get a document by path.
    Get {
        /// Path of the document
        path: String,
        /// Sections to include (comma-separated)
        #[arg(short, long, value_delimiter = ',')]
        include: Vec<String>,
        /// Maximum characters for body content
        #[arg(long, default_value = "12000")]
        max_chars: usize,
    },
    /// Get a specific section from a document.
    Section {
        /// Path of the document
        path: String,
        /// Heading title or anchor slug
        heading: String,
        /// Maximum characters for section content
        #[arg(long, default_value = "5000")]
        max_chars: usize,
    },
    /// Full-text search.
    Search {
        /// Full-text search query
        query: String,
        /// Restrict search to path prefix
        #[arg(short = 'p', long)]
        path_prefix: Option<String>,
        /// Filter by concept types (comma-separated)
        #[arg(short = 'T', long, value_delimiter = ',')]
        types: Vec<String>,
        /// Filter by tags (comma-separated)
        #[arg(short = 'g', long, value_delimiter = ',')]
        tags: Vec<String>,
        /// Maximum results
        #[arg(short, long, default_value = "20")]
        limit: usize,
    },
    /// Structured metadata query with filtering and projection.
    Metadata {
        /// Filter key=value pairs
        #[arg(short, long, value_parser = parse_key_val)]
        filter: Vec<(String, String)>,
        /// Select fields
        #[arg(short, long, value_delimiter = ',')]
        select: Vec<String>,
        /// Maximum rows
        #[arg(short, long, default_value = "100")]
        limit: usize,
    },
    /// Get forward links from a document.
    Links {
        /// Path of the document
        path: String,
    },
    /// Get backlinks to a document.
    Backlinks {
        /// Path of the document
        path: String,
        /// Maximum results
        #[arg(short, long, default_value = "50")]
        limit: usize,
    },
    /// Traverse the link graph from a starting document.
    Traverse {
        /// Starting document path
        start: String,
        /// Link relation types to follow (comma-separated)
        #[arg(short, long, value_delimiter = ',')]
        relations: Vec<String>,
        /// Maximum traversal depth
        #[arg(long, default_value = "3")]
        max_depth: usize,
        /// Maximum nodes to visit
        #[arg(long, default_value = "50")]
        max_nodes: usize,
    },
    /// Validate the index and report issues.
    Validate {
        /// Output JSON report
        #[arg(long)]
        json: bool,
    },
    /// Show index statistics.
    Stats,
    /// Start MCP server.
    Serve {
        /// Root directories to scan
        #[arg(short, long)]
        root: Vec<PathBuf>,
        /// Transport type: stdio (default) or http
        #[arg(long, value_enum, default_value = "stdio")]
        transport: TransportType,
        /// HTTP server host (for http transport)
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
        /// HTTP server port (for http transport)
        #[arg(long, default_value = "3000")]
        port: u16,
    },
    /// Watch for file changes and update index incrementally.
    Watch {
        /// Root directories to watch (default: configured roots)
        #[arg(short, long)]
        root: Vec<PathBuf>,
        /// Skip initial full scan before starting the watcher
        #[arg(long)]
        skip_initial: bool,
        /// Debounce window in milliseconds (default: 500)
        #[arg(long, default_value = "500")]
        debounce: u64,
        /// Full reconciliation interval in seconds (default: 600 = 10min)
        #[arg(long, default_value = "600")]
        reconcile: u64,
    },
}

fn parse_key_val(s: &str) -> Result<(String, String), String> {
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` in `{s}`"))?;
    Ok((s[..pos].to_string(), s[pos + 1..].to_string()))
}
