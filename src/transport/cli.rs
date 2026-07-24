use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "okc",
    about = "Open Knowledge Catalog - index and query markdown knowledge bases"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Scan {
        /// Root directories to scan
        #[arg(short, long)]
        root: Vec<PathBuf>,
    },
    Browse {
        /// Path to browse (default: root)
        path: Option<String>,
        #[arg(short, long, default_value = "1")]
        depth: usize,
        #[arg(short, long, default_value = "100")]
        limit: usize,
    },
    Get {
        /// Path of the document
        path: String,
        #[arg(short, long, value_delimiter = ',')]
        include: Vec<String>,
        #[arg(long, default_value = "12000")]
        max_chars: usize,
    },
    Section {
        /// Path of the document
        path: String,
        /// Heading title or anchor slug
        heading: String,
        #[arg(long, default_value = "5000")]
        max_chars: usize,
    },
    Search {
        /// Full-text search query
        query: String,
        #[arg(short = 'p', long)]
        path_prefix: Option<String>,
        #[arg(short = 'T', long, value_delimiter = ',')]
        types: Vec<String>,
        #[arg(short = 'g', long, value_delimiter = ',')]
        tags: Vec<String>,
        #[arg(short, long, default_value = "20")]
        limit: usize,
    },
    Metadata {
        /// Filter key=value pairs
        #[arg(short, long, value_parser = parse_key_val)]
        filter: Vec<(String, String)>,
        /// Select fields
        #[arg(short, long, value_delimiter = ',')]
        select: Vec<String>,
        #[arg(short, long, default_value = "100")]
        limit: usize,
    },
    Links {
        /// Path of the document
        path: String,
    },
    Backlinks {
        /// Path of the document
        path: String,
        #[arg(short, long, default_value = "50")]
        limit: usize,
    },
    Traverse {
        /// Starting document path
        start: String,
        #[arg(short, long, value_delimiter = ',')]
        relations: Vec<String>,
        #[arg(long, default_value = "3")]
        max_depth: usize,
        #[arg(long, default_value = "50")]
        max_nodes: usize,
    },
    Validate {
        /// Output JSON report
        #[arg(long)]
        json: bool,
    },
    Stats,
    Serve {
        /// Root directories to scan
        #[arg(short, long)]
        root: Vec<PathBuf>,
    },
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
