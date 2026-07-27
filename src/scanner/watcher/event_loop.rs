//! Core event loop and helper functions.
//!
//! Bridges `notify` raw events into debounced `WatchEvent` batches,
//! with gitignore-aware filtering.

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::{Duration, Instant};

use ignore::gitignore::{Gitignore, GitignoreBuilder};
use notify::event::{Event, EventKind};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use tracing::{debug, error, info, warn};

use super::FileWatcher;
use super::WatchEvent;

/// Core event loop: bridges `notify` raw events into debounced `WatchEvent` batches.
pub(crate) fn run_loop(
    output_tx: &mpsc::Sender<WatchEvent>,
    roots: &[PathBuf],
    debounce: Duration,
    reconcile: Duration,
) -> Result<(), anyhow::Error> {
    let (raw_tx, raw_rx) = mpsc::channel::<Result<Event, notify::Error>>();
    let mut watcher = RecommendedWatcher::new(raw_tx, Config::default())?;

    for root in roots {
        let canonical = std::fs::canonicalize(root).unwrap_or_else(|_| root.clone());
        if canonical.exists() {
            info!("Watching root: {:?}", canonical);
            watcher.watch(&canonical, RecursiveMode::Recursive)?;
        } else {
            warn!("Watch root does not exist, skipping: {:?}", canonical);
        }
    }

    if roots.is_empty() {
        warn!("No roots configured — watcher has nothing to monitor");
    }

    let mut gitignore_matchers: Vec<(PathBuf, Gitignore)> = Vec::new();
    for root in roots {
        let canonical = std::fs::canonicalize(root).unwrap_or_else(|_| root.clone());
        let matcher = build_gitignore(&canonical);
        gitignore_matchers.push((canonical, matcher));
    }

    let is_gitignored = |path: &Path| -> bool {
        for (root, gi) in &gitignore_matchers {
            match path.strip_prefix(root) {
                Ok(rel) => {
                    if gi.matched(rel, false).is_ignore() {
                        return true;
                    }
                }
                Err(_) => continue,
            }
        }
        false
    };

    let mut pending: HashSet<PathBuf> = HashSet::new();
    let mut last_flush = Instant::now();
    let mut last_reconcile = Instant::now();

    loop {
        let now = Instant::now();
        let since_flush = now.duration_since(last_flush);
        let since_reconcile = now.duration_since(last_reconcile);

        let until_flush = if pending.is_empty() {
            None
        } else if debounce > since_flush {
            Some(debounce - since_flush)
        } else {
            Some(Duration::ZERO)
        };

        let until_reconcile = if reconcile > since_reconcile {
            Some(reconcile - since_reconcile)
        } else {
            Some(Duration::ZERO)
        };

        let timeout = match (until_flush, until_reconcile) {
            (Some(a), Some(b)) => Some(std::cmp::min(a, b)),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        };

        let raw_event: Option<Result<Event, notify::Error>> = match timeout {
            Some(t) if !t.is_zero() => match raw_rx.recv_timeout(t) {
                Ok(e) => Some(e),
                Err(mpsc::RecvTimeoutError::Timeout) => None,
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    error!("notify channel disconnected");
                    return Ok(());
                }
            },
            Some(_) => raw_rx.try_recv().ok(),
            None => match raw_rx.recv() {
                Ok(e) => Some(e),
                Err(mpsc::RecvError) => {
                    error!("notify channel disconnected");
                    return Ok(());
                }
            },
        };

        if let Some(event) = raw_event {
            match event {
                Ok(ev) => {
                    let paths = extract_paths(&ev);
                    for pb in paths {
                        if pb.extension().is_some_and(|e| e == "md")
                            && !FileWatcher::is_ignored(&pb)
                            && !is_gitignored(&pb)
                        {
                            debug!("Watch event: {} ({ev:?})", pb.display());
                            pending.insert(pb);
                            last_flush = Instant::now();
                        }
                    }
                }
                Err(e) => {
                    warn!("notify error: {e}");
                }
            }
        }

        if last_reconcile.elapsed() >= reconcile {
            info!("Scheduled full reconciliation scan");
            if output_tx.send(WatchEvent::Reconcile).is_err() {
                return Ok(());
            }
            last_reconcile = Instant::now();
        }

        if !pending.is_empty() && last_flush.elapsed() >= debounce {
            let batch: HashSet<PathBuf> = std::mem::take(&mut pending);
            debug!("Flushing {} change(s) after debounce", batch.len());
            if output_tx.send(WatchEvent::Changes(batch)).is_err() {
                return Ok(());
            }
        }
    }
}

/// Extract relevant file paths from a notify Event.
pub(crate) fn extract_paths(event: &Event) -> Vec<PathBuf> {
    match &event.kind {
        EventKind::Any | EventKind::Access(_) | EventKind::Create(_) | EventKind::Modify(_) => {
            event.paths.clone()
        }
        EventKind::Remove(_) => event.paths.clone(),
        EventKind::Other => event.paths.clone(),
    }
}

fn build_gitignore(root: &Path) -> Gitignore {
    let mut builder = GitignoreBuilder::new(root);
    let gitignore_path = root.join(".gitignore");
    if gitignore_path.exists() {
        if let Some(e) = builder.add(gitignore_path) {
            debug!("Could not load .gitignore: {e}");
        }
    }
    builder.build().unwrap_or(Gitignore::empty())
}
