//! File system scanning, change detection, and watching.
//!
//! This module provides:
//!
//! - [`changes`] - [`ChangeDetector`]: Detects added/modified/deleted files between scans
//! - [`walker`] - [`Scanner`]: Parallel filesystem walker for markdown file discovery
//! - [`watcher`] - [`FileWatcher`]: Cross-platform file watcher with debouncing and reconciliation

pub mod changes;
pub mod walker;
pub mod watcher;
