use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::{Duration, Instant};

use ignore::gitignore::{Gitignore, GitignoreBuilder};
use notify::event::{Event, EventKind};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use tracing::{debug, error, info, warn};

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
    pub fn new(roots: Vec<PathBuf>, debounce_ms: u64, reconcile_secs: u64) -> Self {
        Self {
            roots,
            debounce: Duration::from_millis(debounce_ms),
            reconcile: Duration::from_secs(reconcile_secs),
        }
    }

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
                if let Err(e) = run_loop(&tx, &roots, debounce, reconcile) {
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

/// Core event loop: bridges `notify` raw events into debounced `WatchEvent` batches.
fn run_loop(
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
fn extract_paths(event: &Event) -> Vec<PathBuf> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use notify::event::EventAttributes;

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
