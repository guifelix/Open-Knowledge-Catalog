//! Open Knowledge Catalog (OKC) - A markdown knowledge base indexer and query engine.
//!
//! This crate provides the core functionality for indexing, searching, and querying
//! markdown-based knowledge repositories with front-matter metadata, wiki-style links,
//! and graph-based navigation.
//!
//! # Architecture
//!
//! The crate is organized into the following modules:
//!
//! - [`config`] - Configuration types and defaults for the indexer
//! - [`index`] - Storage layer: database, search index, graph store, and migrations
//! - [`model`] - Core data types: documents, front-matter, links, graph structures
//! - [`parser`] - Parsing pipeline: front-matter, markdown, links, YAML
//! - [`scanner`] - File system scanning, change detection, and watching
//! - [`service`] - High-level service API for CLI and MCP server
//! - [`transport`] - Transport layer: CLI commands and MCP (Model Context Protocol) server
//!
//! # Example
//!
//! ```no_run
//! use okc::{config::OkcConfig, service::OkcService};
//!
//! let config = OkcConfig::default();
//! let mut service = OkcService::open(&config)?;
//! let result = service.scan()?;
//! println!("Indexed {} files", result.total_files);
//! # Ok::<(), anyhow::Error>(())
//! ```

pub mod config;
pub mod error;
pub mod index;
pub mod model;
pub mod parser;
pub mod scanner;
pub mod service;
pub mod transport;
