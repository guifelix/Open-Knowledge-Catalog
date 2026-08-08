//! Deterministic, bounded path-suggestion generation.
//!
//! Given a requested repository path that did not resolve, `suggest_paths`
//! derives a small, deterministic set of "did you mean" candidates from the
//! set of paths that exist in the index. Candidate generation is pure,
//! bounded, and never echoes the requested path itself.

/// Maximum number of suggestions returned by [`suggest_paths`].
pub const MAX_SUGGESTIONS: usize = 4;

/// Maximum Optimal String Alignment distance accepted for a suggestion.
///
/// Candidates farther than this from the query are not returned, which keeps
/// suggestion generation responsive on large indexes.
pub const MAX_EDIT_DISTANCE: usize = 3;

/// Generate ranked path suggestions for a query that did not resolve.
///
/// Ranking is deterministic: candidates are ordered by bounded Optimal String
/// Alignment (transposition-aware Levenshtein) distance over the full path,
/// with lexicographic order breaking ties. The requested path itself is never
/// returned, and unsafe queries (empty, traversal, absolute, drive-prefixed,
/// or URL forms) yield an empty result.
pub fn suggest_paths(query: &str, candidates: &[String], max_suggestions: usize) -> Vec<String> {
    if max_suggestions == 0 || !query_is_safe(query) {
        return Vec::new();
    }

    let query_chars: Vec<char> = query.chars().collect();

    let mut scored: Vec<(usize, &str)> = candidates
        .iter()
        .filter(|candidate| *candidate != query)
        .filter_map(|candidate| {
            let distance = osa_edit_distance(
                &query_chars,
                &candidate.chars().collect::<Vec<_>>(),
                MAX_EDIT_DISTANCE,
            );
            (distance <= MAX_EDIT_DISTANCE).then_some((distance, candidate.as_str()))
        })
        .collect();

    scored.sort_by(|left, right| left.0.cmp(&right.0).then_with(|| left.1.cmp(right.1)));

    scored
        .into_iter()
        .take(max_suggestions)
        .map(|(_, path)| path.to_string())
        .collect()
}

/// Reject query forms that must never be used to derive suggestions.
///
/// Returns `false` for empty input, absolute paths (leading `/` or `\`),
/// path traversal (`..` segments), drive-prefixed paths (e.g. `C:\...`), and
/// URL-like input containing a scheme separator (`://`).
fn query_is_safe(query: &str) -> bool {
    if query.is_empty() || query.contains("://") {
        return false;
    }

    let bytes = query.as_bytes();
    if bytes.first() == Some(&b'/') || bytes.first() == Some(&b'\\') {
        return false;
    }
    if bytes.len() >= 2 && bytes[0].is_ascii_alphabetic() && (bytes[1] == b':' || bytes[1] == b'%')
    {
        return false;
    }

    query
        .split('/')
        .all(|segment| segment != ".." && segment != ".")
}

/// Bounded Optimal String Alignment (transposition-aware Levenshtein) distance.
///
/// Returns `cap + 1` when the distance provably exceeds `cap`, so callers can
/// drop far candidates without scanning the full DP matrix. The dynamic
/// programming runs on three rolling rows in a diagonal band around the main
/// diagonal, keeping work `O(n * cap)` per candidate.
fn osa_edit_distance(left: &[char], right: &[char], cap: usize) -> usize {
    let n = left.len();
    let m = right.len();
    let exceeded = cap + 1;

    if n.abs_diff(m) > cap {
        return exceeded;
    }
    if n == 0 {
        return m.min(exceeded);
    }
    if m == 0 {
        return n.min(exceeded);
    }

    let width = m + 1;
    let mut two_previous: Vec<usize> = (0..=m).map(|value| value.min(exceeded)).collect();
    let mut previous: Vec<usize> = (0..=m).map(|value| value.min(exceeded)).collect();

    for i in 1..=n {
        let mut current = vec![exceeded; width];
        current[0] = i.min(exceeded);

        let low = i.saturating_sub(cap).max(1);
        let high = (i + cap).min(m);
        for j in low..=high {
            let substitution = if left[i - 1] == right[j - 1] { 0 } else { 1 };
            let mut distance = (previous[j] + 1)
                .min(current[j - 1] + 1)
                .min(previous[j - 1] + substitution)
                .min(exceeded);
            if i > 1 && j > 1 && left[i - 1] == right[j - 2] && left[i - 2] == right[j - 1] {
                distance = distance.min(two_previous[j - 2] + 1).min(exceeded);
            }
            current[j] = distance;
        }

        two_previous = previous;
        previous = current;
    }

    previous[m]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn paths() -> Vec<String> {
        vec![
            "metrics/monthly-revenue.md".to_string(),
            "metrics/churn-rate.md".to_string(),
            "metrics/quarterly-revenue.md".to_string(),
            "docs/getting-started.md".to_string(),
        ]
    }

    #[test]
    fn suggest_ranks_closest_first() {
        let suggestions = suggest_paths("metrics/monthly-revenu.md", &paths(), MAX_SUGGESTIONS);
        assert_eq!(
            suggestions.first().map(String::as_str),
            Some("metrics/monthly-revenue.md")
        );
    }

    #[test]
    fn suggest_bounds_max_count() {
        let candidates = (0..10)
            .map(|index| format!("metrics/revenue-{index}.md"))
            .collect::<Vec<_>>();
        let suggestions = suggest_paths("metrics/revenue.md", &candidates, 4);
        assert!(suggestions.len() <= 4);
        assert!(!suggestions.is_empty());
    }

    #[test]
    fn suggest_never_returns_query_itself() {
        let mut candidates = paths();
        candidates.push("metrics/monthly-revenu.md".to_string());
        let suggestions = suggest_paths("metrics/monthly-revenu.md", &candidates, MAX_SUGGESTIONS);
        assert!(!suggestions
            .iter()
            .any(|path| path == "metrics/monthly-revenu.md"));
    }

    #[test]
    fn suggest_tie_break_lexicographic() {
        let candidates = vec!["b.md".to_string(), "a.md".to_string()];
        let suggestions = suggest_paths("c.md", &candidates, MAX_SUGGESTIONS);
        assert_eq!(suggestions, vec!["a.md".to_string(), "b.md".to_string()]);
    }

    #[test]
    fn suggest_no_match_returns_empty() {
        let suggestions = suggest_paths("unrelated/topic.md", &paths(), MAX_SUGGESTIONS);
        assert!(suggestions.is_empty());
    }

    #[test]
    fn suggest_rejects_traversal() {
        let suggestions = suggest_paths("../../etc/passwd", &paths(), MAX_SUGGESTIONS);
        assert!(suggestions.is_empty());
    }

    #[test]
    fn suggest_rejects_dot_segment() {
        let suggestions = suggest_paths("./metrics/revenue.md", &paths(), MAX_SUGGESTIONS);
        assert!(suggestions.is_empty());
    }

    #[test]
    fn suggest_rejects_absolute_path() {
        let suggestions = suggest_paths("/metrics/revenue.md", &paths(), MAX_SUGGESTIONS);
        assert!(suggestions.is_empty());
    }

    #[test]
    fn suggest_rejects_drive_prefixed_path() {
        let suggestions = suggest_paths("C:\\metrics\\revenue.md", &paths(), MAX_SUGGESTIONS);
        assert!(suggestions.is_empty());
    }

    #[test]
    fn suggest_rejects_url_form() {
        let suggestions = suggest_paths("file:///metrics/revenue.md", &paths(), MAX_SUGGESTIONS);
        assert!(suggestions.is_empty());
    }

    #[test]
    fn suggest_rejects_empty_query() {
        let suggestions = suggest_paths("", &paths(), MAX_SUGGESTIONS);
        assert!(suggestions.is_empty());
    }

    #[test]
    fn suggest_zero_max_returns_empty() {
        let suggestions = suggest_paths("metrics/revenue.md", &paths(), 0);
        assert!(suggestions.is_empty());
    }

    #[test]
    fn edit_distance_bounds_excees_cap() {
        let distance = osa_edit_distance(
            &"short".chars().collect::<Vec<_>>(),
            &"this is a very long candidate".chars().collect::<Vec<_>>(),
            3,
        );
        assert_eq!(distance, 4);
    }
}
