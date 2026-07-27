//! Cross-platform filesystem watcher with debouncing and reconciliation.
//!
//! [`FileWatcher`] monitors repository roots for changes to markdown files,
//! providing:
//! - Debounced event batching (configurable window)
//! - Editor temporary file filtering (`.swp`, `~`, `.tmp`, etc.)
//! - Gitignore-aware exclusion
//! - Periodic full reconciliation to catch missed events
//! - `.md` file extension filtering

pub(crate) mod event_loop;

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::Duration;

use tracing::error;

/// Events emitted by the file watcher.
#[derive(Debug, Clone)]
pub enum WatchEvent {
    /// A batch of file paths that changed (debounced, filtered).
    Changes(HashSet<PathBuf>),
    /// Periodic full reconciliation signal.
    Reconcile,
}

/// Cross-platform filesystem watcher with debouncing, editor-temp-file filtering,
/// and periodic full-reconciliation support.
pub struct FileWatcher {
    roots: Vec<PathBuf>,
    debounce: Duration,
    reconcile: Duration,
}

impl Default for FileWatcher {
    fn default() -> Self {
        Self {
            roots: vec![],
            debounce: Duration::from_millis(500),
            reconcile: Duration::from_secs(600),
        }
    }
}

impl FileWatcher {
    /// Create a new file watcher with custom roots and timing.
    ///
    /// - `roots`: Directories to watch recursively
    /// - `debounce_ms`: Debounce window in milliseconds (default: 500)
    /// - `reconcile_secs`: Full reconciliation interval in seconds (default: 600)
    pub fn new(roots: Vec<PathBuf>, debounce_ms: u64, reconcile_secs: u64) -> Self {
        Self {
            roots,
            debounce: Duration::from_millis(debounce_ms),
            reconcile: Duration::from_secs(reconcile_secs),
        }
    }

    /// Create a watcher with default timing from roots.
    #[allow(dead_code)]
    pub fn from_roots(roots: Vec<PathBuf>) -> Self {
        Self {
            roots,
            ..Default::default()
        }
    }

    /// Start watching. Returns a receiver that yields debounced change batches
    /// and periodic reconcile signals. Spawns a background thread.
    pub fn start(&self) -> Result<mpsc::Receiver<WatchEvent>, anyhow::Error> {
        let (tx, rx) = mpsc::channel();
        let roots = self.roots.clone();
        let debounce = self.debounce;
        let reconcile = self.reconcile;

        std::thread::Builder::new()
            .name("okc-watcher".into())
            .spawn(move || {
                if let Err(e) = event_loop::run_loop(&tx, &roots, debounce, reconcile) {
                    error!("File watcher terminated with error: {e}");
                }
            })
            .map_err(|e| anyhow::anyhow!("Failed to spawn watcher thread: {}", e))?;

        Ok(rx)
    }

    /// Returns `true` if the path matches an editor-temp or build-artifact pattern
    /// that should never trigger a re-index.
    pub fn is_ignored(path: &Path) -> bool {
        let path_str = path.to_string_lossy();

        if path_str.contains("/.git/")
            || path_str.contains("/vendor/")
            || path_str.contains("/node_modules/")
            || path_str.contains("/target/")
        {
            return true;
        }

        let file_name = match path.file_name().and_then(|n| n.to_str()) {
            Some(n) => n,
            None => return false,
        };

        if file_name.starts_with('.') {
            return true;
        }

        if file_name.ends_with('~') {
            return true;
        }

        if file_name.ends_with(".swp") || file_name.ends_with(".swo") || file_name.ends_with(".swx")
        {
            return true;
        }

        if file_name.starts_with('#') && file_name.ends_with('#') {
            return true;
        }
        if file_name.starts_with(".#") {
            return true;
        }

        let lower = file_name.to_lowercase();
        lower.ends_with(".tmp") || lower.ends_with(".bak") || lower.ends_with(".crash")
    }
}

#[cfg(test)]
mod tests {
    use super::event_loop::extract_paths;
    use super::*;
    use notify::event::{Event, EventAttributes, EventKind};

    #[test]
    fn test_is_ignored_git_dir() {
        assert!(FileWatcher::is_ignored(Path::new(
            "/repo/.git/objects/abc123"
        )));
    }

    #[test]
    fn test_is_ignored_vendor() {
        assert!(FileWatcher::is_ignored(Path::new(
            "/repo/vendor/bundle/gems"
        )));
    }

    #[test]
    fn test_is_ignored_node_modules() {
        assert!(FileWatcher::is_ignored(Path::new("/repo/node_modules/pkg")));
    }

    #[test]
    fn test_is_ignored_target() {
        assert!(FileWatcher::is_ignored(Path::new(
            "/repo/target/debug/build"
        )));
    }

    #[test]
    fn test_is_ignored_hidden_file() {
        assert!(FileWatcher::is_ignored(Path::new("/repo/.hidden.md")));
    }

    #[test]
    fn test_is_ignored_vim_swap() {
        assert!(FileWatcher::is_ignored(Path::new("/repo/file.md.swp")));
        assert!(FileWatcher::is_ignored(Path::new("/repo/file.md.swo")));
        assert!(FileWatcher::is_ignored(Path::new("/repo/file.md.swx")));
    }

    #[test]
    fn test_is_ignored_tilde_backup() {
        assert!(FileWatcher::is_ignored(Path::new("/repo/file.md~")));
    }

    #[test]
    fn test_is_ignored_emacs_autosave() {
        assert!(FileWatcher::is_ignored(Path::new("/repo/#file.md#")));
    }

    #[test]
    fn test_is_ignored_emacs_lock() {
        assert!(FileWatcher::is_ignored(Path::new("/repo/.#file.md")));
    }

    #[test]
    fn test_is_ignored_tmp_file() {
        assert!(FileWatcher::is_ignored(Path::new("/repo/file.tmp")));
        assert!(FileWatcher::is_ignored(Path::new("/repo/file.bak")));
    }

    #[test]
    fn test_is_not_ignored_markdown() {
        assert!(!FileWatcher::is_ignored(Path::new("/repo/readme.md")));
        assert!(!FileWatcher::is_ignored(Path::new("/repo/docs/guide.md")));
        assert!(!FileWatcher::is_ignored(Path::new(
            "/repo/metrics/monthly-revenue.md"
        )));
    }

    #[test]
    fn test_is_not_ignored_non_markdown() {
        assert!(!FileWatcher::is_ignored(Path::new("/repo/Cargo.toml")));
        assert!(!FileWatcher::is_ignored(Path::new("/repo/src/lib.rs")));
    }

    fn make_event(kind: EventKind, path: PathBuf) -> Event {
        Event {
            kind,
            paths: vec![path],
            attrs: EventAttributes::default(),
        }
    }

    #[test]
    fn test_extract_paths_create() {
        let p = PathBuf::from("/repo/file.md");
        let ev = make_event(
            EventKind::Create(notify::event::CreateKind::File),
            p.clone(),
        );
        let paths = extract_paths(&ev);
        assert_eq!(paths, vec![p]);
    }

    #[test]
    fn test_extract_paths_remove() {
        let p = PathBuf::from("/repo/file.md");
        let ev = make_event(
            EventKind::Remove(notify::event::RemoveKind::File),
            p.clone(),
        );
        let paths = extract_paths(&ev);
        assert_eq!(paths, vec![p]);
    }

    #[test]
    fn test_extract_paths_modify() {
        let p = PathBuf::from("/repo/file.md");
        let ev = make_event(
            EventKind::Modify(notify::event::ModifyKind::Data(
                notify::event::DataChange::Any,
            )),
            p.clone(),
        );
        let paths = extract_paths(&ev);
        assert_eq!(paths, vec![p]);
    }
}
