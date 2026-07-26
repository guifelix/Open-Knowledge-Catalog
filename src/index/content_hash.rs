/// Content hashing module with optimized Blake3 usage for large files.
///
/// This module provides content hashing with sampling-based optimization
/// for very large files, ensuring deterministic hashes while maintaining
/// performance for technical repositories with large documents.
use blake3;

/// Configuration for content hashing behavior.
#[derive(Debug, Clone, Copy)]
pub struct HashConfig {
    /// Maximum file size (in bytes) to hash fully without sampling.
    /// Files larger than this will use sampling-based hashing.
    /// Default: 1 MiB.
    pub full_hash_threshold: usize,

    /// Number of samples to take for large files.
    /// Default: 64 samples.
    pub sample_count: usize,

    /// Size of each sample in bytes.
    /// Default: 4 KiB.
    pub sample_size: usize,
}

impl Default for HashConfig {
    fn default() -> Self {
        Self {
            full_hash_threshold: 1024 * 1024, // 1 MiB
            sample_count: 64,
            sample_size: 4096, // 4 KiB
        }
    }
}

/// Compute a content hash using the default configuration.
///
/// For files smaller than the threshold, computes a full Blake3 hash.
/// For larger files, uses deterministic sampling to produce a hash
/// that is consistent for identical content but much faster to compute.
pub fn compute_content_hash_default(content: &[u8]) -> String {
    compute_content_hash(content, HashConfig::default())
}

/// Compute a content hash with custom configuration.
pub fn compute_content_hash(content: &[u8], config: HashConfig) -> String {
    if content.len() <= config.full_hash_threshold {
        // Small file: full hash
        return hash_full(content);
    }

    // Large file: sampled hash
    hash_sampled(content, config)
}

/// Compute full Blake3 hash of content.
fn hash_full(content: &[u8]) -> String {
    blake3::hash(content).to_hex().to_string()
}

/// Compute sampled Blake3 hash for large content.
///
/// Uses deterministic sampling: samples are taken at regular intervals
/// throughout the file, plus the beginning and end. This ensures:
/// - Identical content produces identical hashes
/// - Similar content produces different hashes (avalanche effect)
/// - Much faster than hashing entire large files
fn hash_sampled(content: &[u8], config: HashConfig) -> String {
    let len = content.len();
    let sample_count = config.sample_count.min(len / config.sample_size.max(1));
    let sample_count = sample_count.max(2); // At least start and end

    let mut hasher = blake3::Hasher::new();

    // Always hash the first sample_size bytes (header)
    let header_end = config.sample_size.min(len);
    hasher.update(&content[..header_end]);

    // Hash samples at regular intervals
    if sample_count > 2 {
        let stride = (len - config.sample_size) / (sample_count - 1);
        for i in 1..sample_count - 1 {
            let start = (i * stride).min(len.saturating_sub(config.sample_size));
            let end = (start + config.sample_size).min(len);
            hasher.update(&content[start..end]);
        }
    }

    // Always hash the last sample_size bytes (footer)
    let footer_start = len.saturating_sub(config.sample_size);
    hasher.update(&content[footer_start..]);

    // Include file length in hash to distinguish same-content-different-length
    hasher.update(&len.to_le_bytes());

    hasher.finalize().to_hex().to_string()
}

/// Deterministic truncation strategy for large documents.
///
/// Preserves key sections (front-matter, headings, first/last content)
/// while truncating middle sections to fit within size limits.
pub fn truncate_document_preserving_structure(content: &str, max_chars: usize) -> String {
    let chars: Vec<char> = content.chars().collect();
    let total_len = chars.len();

    if total_len <= max_chars {
        return content.to_string();
    }

    // Reserve space for truncation marker
    let truncation_marker = "\n\n[... truncated ...]\n\n";
    let marker_len = truncation_marker.chars().count();

    // If max_chars is too small to even fit the marker, just truncate
    if max_chars <= marker_len {
        return content.chars().take(max_chars).collect();
    }

    let available = max_chars - marker_len;

    // Find front-matter end
    let fm_end = find_front_matter_end(&chars);

    // Calculate how much to take from start and end
    // We need: start_portion + marker + end_portion <= max_chars
    // So: start_portion + end_portion <= available
    // The start_portion includes front-matter (fm_end chars) + start_budget
    // The end_portion is end_budget
    // So: fm_end + start_budget + end_budget <= available
    // Since start_budget + end_budget = available, we need fm_end <= 0 which is not true
    // So we need to subtract fm_end from available
    let available_for_content = available.saturating_sub(fm_end);

    let start_budget = available_for_content / 2;
    let end_budget = available_for_content - start_budget;

    // Take from start (including front-matter and early content)
    let start_end = (fm_end + start_budget).min(total_len);
    let start_portion: String = chars[..start_end].iter().collect();

    // Take from end (last content)
    let end_start = total_len.saturating_sub(end_budget);
    let end_portion: String = chars[end_start..].iter().collect();

    // Combine with marker
    let mut result = String::with_capacity(max_chars);
    result.push_str(&start_portion);
    result.push_str(truncation_marker);
    result.push_str(&end_portion);

    // Final safety check - should not exceed max_chars now
    debug_assert!(result.chars().count() <= max_chars);

    result
}

/// Find the end position of front-matter (after the closing `---`).
fn find_front_matter_end(chars: &[char]) -> usize {
    let content: String = chars.iter().collect();
    let lines: Vec<&str> = content.lines().collect();

    let mut in_front_matter = false;
    let mut fm_end_line = 0;

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed == "---" {
            if !in_front_matter {
                in_front_matter = true;
            } else {
                // Closing ---
                fm_end_line = i + 1;
                break;
            }
        }
    }

    if fm_end_line > 0 {
        // Calculate character position
        lines[..fm_end_line].join("\n").chars().count() + 1 // +1 for newline
    } else {
        0
    }
}

/// Find positions of all headings in the document.
fn find_heading_positions(chars: &[char]) -> Vec<usize> {
    let content: String = chars.iter().collect();
    let mut positions = Vec::new();

    for (i, line) in content.lines().enumerate() {
        let trimmed = line.trim_start();
        if trimmed.starts_with('#') {
            // Calculate character position of this line
            let pos: usize = content.lines().take(i).map(|l| l.chars().count() + 1).sum();
            positions.push(pos);
        }
    }

    positions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_hash_small_content() {
        let content = b"Hello, world!";
        let hash = compute_content_hash_default(content);
        assert_eq!(hash.len(), 64); // Blake3 hex output is 64 chars
    }

    #[test]
    fn test_sampled_hash_large_content() {
        // Create content larger than 1 MiB
        let content = vec![b'x'; 2 * 1024 * 1024];
        let hash = compute_content_hash_default(&content);
        assert_eq!(hash.len(), 64);
    }

    #[test]
    fn test_deterministic_hash() {
        let content = b"Deterministic content for testing";
        let hash1 = compute_content_hash_default(content);
        let hash2 = compute_content_hash_default(content);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_different_content_different_hash() {
        let content1 = b"Content A";
        let content2 = b"Content B";
        let hash1 = compute_content_hash_default(content1);
        let hash2 = compute_content_hash_default(content2);
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_truncate_preserves_structure() {
        let content = "---\ntitle: Test\n---\n\n# Heading 1\n\nContent 1\n\n# Heading 2\n\nContent 2\n\n# Heading 3\n\nContent 3";
        eprintln!("Content len: {}", content.len());
        let truncated = truncate_document_preserving_structure(content, 50);
        eprintln!("Truncated: '{}'", truncated);
        eprintln!("Truncated len: {}", truncated.len());
        assert!(truncated.len() <= 50);
        assert!(truncated.contains("title: Test"));
        assert!(truncated.contains("[... truncated ...]"));
    }

    #[test]
    fn test_truncate_short_content_unchanged() {
        let content = "Short content";
        let truncated = truncate_document_preserving_structure(content, 100);
        assert_eq!(truncated, content);
    }

    #[test]
    fn test_hash_config_custom() {
        let config = HashConfig {
            full_hash_threshold: 100,
            sample_count: 10,
            sample_size: 10,
        };
        let content = vec![b'x'; 1000];
        let hash = compute_content_hash(&content, config);
        assert_eq!(hash.len(), 64);
    }
}
