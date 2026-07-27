//! Property-based tests using proptest

#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::map_unwrap_or)]

use okc::model::document::FileRecord;
use okc::model::Link as ModelLink;
use okc::parser::frontmatter::FrontMatterExtractor;
use okc::parser::link_utils::{normalize_path, split_anchor};
use okc::parser::links::LinkResolver;
use okc::parser::yaml::YamlParser;
use okc::scanner::changes::{ChangeDetector, FileChanges};
use proptest::prelude::*;
use proptest::test_runner::TestCaseResult;
use std::path::Path;

fn prop_frontmatter_extractor_never_panics(input: Vec<u8>) -> TestCaseResult {
    let extractor = FrontMatterExtractor::new(4096);
    let _ = extractor.extract(&input);
    Ok(())
}

fn prop_frontmatter_extractor_small_limit(input: Vec<u8>) -> TestCaseResult {
    let extractor = FrontMatterExtractor::new(10);
    let _ = extractor.extract(&input);
    Ok(())
}

fn prop_yaml_parser_never_panics(input: String) -> TestCaseResult {
    let _ = YamlParser::parse(&input);
    Ok(())
}

fn prop_link_resolution_never_panics(source: String, target: String) -> TestCaseResult {
    let _ = LinkResolver::resolve(&source, &target);
    Ok(())
}

fn prop_external_urls_unchanged(url: String) -> TestCaseResult {
    let source = "metrics/test.md";
    let result = LinkResolver::resolve(source, &url);
    prop_assert_eq!(result, url);
    Ok(())
}

fn prop_relative_path_resolution(dir1: String, dir2: String, file: String) -> TestCaseResult {
    let source = format!("{}/{}", dir1, file);
    let target = format!("{}/{}", dir2, file);
    let result = LinkResolver::resolve(&source, &target);
    // Target is resolved relative to source's parent directory (dir1/)
    let expected = format!("{}/{}/{}", dir1, dir2, file);
    prop_assert_eq!(result, expected);
    Ok(())
}

fn prop_parent_directory_traversal(file: String) -> TestCaseResult {
    let source = format!("metrics/{}", file);
    let target = format!("../datasets/{}", file);
    let result = LinkResolver::resolve(&source, &target);
    prop_assert_eq!(result, format!("datasets/{}", file));
    Ok(())
}

fn prop_path_normalization_never_panics(input: String) -> TestCaseResult {
    let path = Path::new(&input);
    let _ = normalize_path(path);
    Ok(())
}

fn prop_path_normalization_dots(segments: Vec<String>, dots: Vec<String>) -> TestCaseResult {
    let mut path_parts = segments.clone();
    path_parts.extend(dots.iter().map(|s| s.to_string()));
    let path_str = path_parts.join("/");
    let path = Path::new(&path_str);
    let normalized = normalize_path(path);

    // If normalization returns None, it means path traversal was detected
    // which is valid behavior - we just verify it doesn't panic
    if let Some(norm) = normalized {
        let has_dots = norm
            .split('/')
            .any(|s| s == "." || (s == ".." && !norm.starts_with("../")));
        prop_assert!(!has_dots);
    }
    Ok(())
}

fn prop_link_anchor_handling(base: String, anchor: String) -> TestCaseResult {
    let source = format!("{}.md", base);
    let target = format!("{}.md#{}", base, anchor);

    let link = ModelLink {
        raw: target.clone(),
        target: target.clone(),
        target_anchor: Some(anchor.clone()),
        is_external: false,
        exists_in_repository: true,
    };

    let resolved = LinkResolver::resolve_links(&source, &[link], &[]);
    prop_assert_eq!(resolved.len(), 1);
    prop_assert_eq!(&resolved[0].target_anchor, &Some(anchor));
    Ok(())
}

fn prop_utf8_validation_in_frontmatter(invalid_utf8: Vec<u8>) -> TestCaseResult {
    // Only use bytes that are NEVER valid UTF-8
    // 0xFF is never valid in UTF-8
    let invalid_bytes: Vec<u8> = invalid_utf8
        .iter()
        .map(|b| if *b == 0xFF { *b } else { 0xFF })
        .collect();
    let mut input = b"---\n".to_vec();
    input.extend_from_slice(&invalid_bytes);
    input.extend_from_slice(b"\n---\nBody");

    let extractor = FrontMatterExtractor::new(4096);
    let result = extractor.extract(&input);

    prop_assert!(result.is_err());
    Ok(())
}

fn prop_windows_line_endings(yaml_lines: Vec<String>, body: String) -> TestCaseResult {
    let yaml = yaml_lines.join("\r\n");
    let input = format!("---\r\n{}\r\n---\r\n{}", yaml, body);
    let extractor = FrontMatterExtractor::new(4096);
    let result = extractor.extract(input.as_bytes());

    prop_assert!(result.is_ok());
    if let Ok(Some((_, extracted))) = result {
        prop_assert!(extracted.contains(&yaml_lines[0]));
    }
    Ok(())
}

fn prop_nested_path_resolution(base: String, subdirs: Vec<String>, file: String) -> TestCaseResult {
    let source = format!("{}/{}/{}", base, subdirs.join("/"), file);
    let target = format!("../../other/{}", file);
    let result = LinkResolver::resolve(&source, &target);

    let source_path = Path::new(&source);
    let parent = source_path.parent().unwrap_or(Path::new(""));
    let expected = normalize_path(&parent.join(&target))
        .map(|s| s.replace('\\', "/"))
        .unwrap_or_else(|| "INVALID_PATH_TRAVERSAL".to_string());
    prop_assert_eq!(result, expected);
    Ok(())
}

fn prop_link_existence_check(files: Vec<String>, target: String) -> TestCaseResult {
    let exists = LinkResolver::check_exists(&target, &files);
    let expected = files.iter().any(|f| f == &target);
    prop_assert_eq!(exists, expected);
    Ok(())
}

fn prop_yaml_tags_sequence(tags: Vec<String>) -> TestCaseResult {
    let yaml = format!(
        "tags:\n{}\n",
        tags.iter()
            .map(|t| format!("  - {}", t))
            .collect::<Vec<_>>()
            .join("\n")
    );
    let result = YamlParser::parse(&yaml);

    if let Ok(fm) = result {
        prop_assert_eq!(fm.tags, tags);
    }
    Ok(())
}

fn prop_custom_fields_preserved(key: String, value: String) -> TestCaseResult {
    let yaml = format!("{}:\n  {}\n", key, value);
    let result = YamlParser::parse(&yaml);

    if let Ok(fm) = result {
        if !["type", "title", "description", "tags"].contains(&key.as_str()) {
            prop_assert!(fm.custom.contains_key(&key));
        }
    }
    Ok(())
}

fn prop_size_limit_enforcement(yaml_content: Vec<String>, body: String) -> TestCaseResult {
    // Join and ensure it's large enough
    let large_yaml: String = yaml_content.join("\n").repeat(10); // Make it definitely large
    let input = format!("---\n{}\n---\n{}", large_yaml, body);
    let extractor = FrontMatterExtractor::new(100);
    let result = extractor.extract(input.as_bytes());

    prop_assert!(result.is_err());
    Ok(())
}

fn prop_bom_handling(yaml_content: String, body: String) -> TestCaseResult {
    let bom = [0xEFu8, 0xBB, 0xBF];
    let mut input = Vec::new();
    input.extend_from_slice(&bom);
    input.extend_from_slice(b"---\n");
    input.extend_from_slice(yaml_content.as_bytes());
    input.extend_from_slice(b"\n---\n");
    input.extend_from_slice(body.as_bytes());

    let extractor = FrontMatterExtractor::new(4096);
    let result = extractor.extract(&input);

    prop_assert!(result.is_ok());
    if let Ok(Some((_, extracted))) = result {
        prop_assert!(extracted.contains(yaml_content.trim_start_matches('\n')));
    }
    Ok(())
}

fn prop_multiple_delimiters(content1: String, content2: String, body: String) -> TestCaseResult {
    let input = format!("---\n{}\n---\n{}\n---\n{}", content1, content2, body);
    let extractor = FrontMatterExtractor::new(4096);
    let result = extractor.extract(input.as_bytes());

    prop_assert!(result.is_ok());
    if let Ok(Some((_, extracted))) = result {
        prop_assert!(extracted.contains(content1.trim_start_matches('\n')));
    }
    Ok(())
}

proptest! {
    #[test]
    fn frontmatter_extractor_never_panics(input in any::<Vec<u8>>()) {
        prop_frontmatter_extractor_never_panics(input)?;
    }

    #[test]
    fn frontmatter_extractor_small_limit(input in any::<Vec<u8>>()) {
        prop_frontmatter_extractor_small_limit(input)?;
    }

    // YAML parser: use printable ASCII to avoid slow saphyr paths on unicode
    #[test]
    fn yaml_parser_never_panics(input in "[a-zA-Z0-9 ]{0,100}") {
        prop_yaml_parser_never_panics(input)?;
    }

    #[test]
    fn link_resolution_never_panics(source in ".{0,100}", target in ".{0,100}") {
        prop_link_resolution_never_panics(source, target)?;
    }

    #[test]
    fn external_urls_unchanged(url in "https?://[a-zA-Z0-9./?=_%:-]*") {
        prop_external_urls_unchanged(url)?;
    }

    #[test]
    fn relative_path_resolution(
        dir1 in "[a-z]+",
        dir2 in "[a-z]+",
        file in "[a-z]+\\.md"
    ) {
        prop_relative_path_resolution(dir1, dir2, file)?;
    }

    #[test]
    fn parent_directory_traversal(file in "[a-z]+\\.md") {
        prop_parent_directory_traversal(file)?;
    }

    #[test]
    fn path_normalization_never_panics(input in ".{0,200}") {
        prop_path_normalization_never_panics(input)?;
    }

    #[test]
    fn path_normalization_dots(
        segments in prop::collection::vec("[a-z]+", 1..5),
        dots in prop::collection::vec(prop::sample::select(vec![String::from("."), String::from("..")]), 0..3)
    ) {
        prop_path_normalization_dots(segments, dots)?;
    }

    #[test]
    fn link_anchor_handling(
        base in "[a-z/]+",
        anchor in "[a-z0-9-]+"
    ) {
        prop_link_anchor_handling(base, anchor)?;
    }

    #[test]
    fn utf8_validation_in_frontmatter(invalid_utf8 in prop::collection::vec(0xFFu8..=0xFFu8, 1..100)) {
        prop_utf8_validation_in_frontmatter(invalid_utf8)?;
    }

    #[test]
    fn windows_line_endings(
        yaml_lines in prop::collection::vec("[a-z: ]+", 1..5),
        body in ".{0,100}"
    ) {
        prop_windows_line_endings(yaml_lines, body)?;
    }

    #[test]
    fn nested_path_resolution(
        base in "[a-z]+",
        subdirs in prop::collection::vec("[a-z]+", 1..3),
        file in "[a-z]+\\.md"
    ) {
        prop_nested_path_resolution(base, subdirs, file)?;
    }

    #[test]
    fn link_existence_check(
        files in prop::collection::vec("[a-z/]+\\.md", 1..10),
        target in "[a-z/]+\\.md"
    ) {
        prop_link_existence_check(files, target)?;
    }

    #[test]
    fn yaml_tags_sequence(
        tags in prop::collection::vec("[a-z]+", 1..5)
    ) {
        prop_yaml_tags_sequence(tags)?;
    }

    #[test]
    fn custom_fields_preserved(
        key in "[a-z][a-z0-9_]*",
        value in ".{0,100}"
    ) {
        prop_custom_fields_preserved(key, value)?;
    }

    #[test]
    fn size_limit_enforcement(
        yaml_content in prop::collection::vec("[a-z]{10,20}", 1..5),
        body in ".{0,100}"
    ) {
        prop_size_limit_enforcement(yaml_content, body)?;
    }

    #[test]
    fn bom_handling(
        yaml_content in ".{0,100}",
        body in ".{0,100}"
    ) {
        prop_bom_handling(yaml_content, body)?;
    }

    #[test]
    fn multiple_delimiters(
        content1 in ".{0,50}",
        content2 in ".{0,50}",
        body in ".{0,100}"
    ) {
        prop_multiple_delimiters(content1, content2, body)?;
    }
}

// Property tests for change detection edge cases
fn prop_change_detector_identical_files_unchanged(
    mut files: Vec<(String, u64, i64)>,
) -> TestCaseResult {
    // Deduplicate by path — ChangeDetector operates on path identity
    files.sort_by(|a, b| a.0.cmp(&b.0));
    files.dedup_by(|a, b| a.0 == b.0);
    let current: Vec<FileRecord> = files
        .iter()
        .map(|(path, size, modified_at)| FileRecord {
            path: path.clone(),
            absolute_path: path.clone(),
            size: *size,
            modified_at: *modified_at,
        })
        .collect();
    let previous = current.clone();

    let changes = ChangeDetector::detect(&current, &previous);

    prop_assert_eq!(changes.added.len(), 0);
    prop_assert_eq!(changes.modified.len(), 0);
    prop_assert_eq!(changes.deleted.len(), 0);
    prop_assert_eq!(changes.unchanged.len(), current.len());
    Ok(())
}

fn prop_change_detector_new_files_added(
    mut current_files: Vec<(String, u64, i64)>,
    mut new_files: Vec<(String, u64, i64)>,
) -> TestCaseResult {
    // Deduplicate by path — ChangeDetector operates on path identity
    current_files.sort_by(|a, b| a.0.cmp(&b.0));
    current_files.dedup_by(|a, b| a.0 == b.0);
    new_files.sort_by(|a, b| a.0.cmp(&b.0));
    new_files.dedup_by(|a, b| a.0 == b.0);
    // Ensure no path overlap
    let current_paths: std::collections::HashSet<_> =
        current_files.iter().map(|(p, _, _)| p).collect();
    let new_paths: std::collections::HashSet<_> = new_files.iter().map(|(p, _, _)| p).collect();
    if !current_paths.is_disjoint(&new_paths) {
        return Ok(()); // Skip overlapping paths
    }

    let current: Vec<FileRecord> = current_files
        .iter()
        .map(|(path, size, modified_at)| FileRecord {
            path: path.clone(),
            absolute_path: path.clone(),
            size: *size,
            modified_at: *modified_at,
        })
        .collect();

    let mut previous = current.clone();
    for (path, size, modified_at) in &new_files {
        previous.push(FileRecord {
            path: path.clone(),
            absolute_path: path.clone(),
            size: *size,
            modified_at: *modified_at,
        });
    }

    let changes = ChangeDetector::detect(&previous, &current);

    prop_assert_eq!(changes.added.len(), new_files.len());
    prop_assert_eq!(changes.modified.len(), 0);
    prop_assert_eq!(changes.deleted.len(), 0);
    Ok(())
}

fn prop_change_detector_deleted_files(
    mut current_files: Vec<(String, u64, i64)>,
    deleted_count: usize,
) -> TestCaseResult {
    if current_files.is_empty() {
        return Ok(());
    }

    // Deduplicate by path
    current_files.sort_by(|a, b| a.0.cmp(&b.0));
    current_files.dedup_by(|a, b| a.0 == b.0);

    let delete_count = deleted_count.min(current_files.len());
    let mut previous = Vec::new();
    let mut current = Vec::new();

    for (i, (path, size, modified_at)) in current_files.iter().enumerate() {
        let record = FileRecord {
            path: path.clone(),
            absolute_path: path.clone(),
            size: *size,
            modified_at: *modified_at,
        };
        if i < delete_count {
            previous.push(record);
        } else {
            previous.push(record.clone());
            current.push(record);
        }
    }

    let changes = ChangeDetector::detect(&current, &previous);

    prop_assert_eq!(changes.deleted.len(), delete_count);
    prop_assert_eq!(changes.added.len(), 0);
    prop_assert_eq!(changes.modified.len(), 0);
    Ok(())
}

fn prop_change_detector_modified_files(
    mut files: Vec<(String, u64, i64)>,
    mut modified_indices: Vec<usize>,
) -> TestCaseResult {
    if files.is_empty() {
        return Ok(());
    }

    // Deduplicate by path — duplicates break index-based modified tracking
    files.sort_by(|a, b| a.0.cmp(&b.0));
    files.dedup_by(|a, b| a.0 == b.0);

    // Deduplicate and clamp indices
    modified_indices.sort();
    modified_indices.dedup();
    let expected_modified = modified_indices
        .iter()
        .filter(|&&i| i < files.len())
        .count();

    let mut current = Vec::new();
    let mut previous = Vec::new();

    for (i, (path, size, modified_at)) in files.iter().enumerate() {
        let record = FileRecord {
            path: path.clone(),
            absolute_path: path.clone(),
            size: *size,
            modified_at: *modified_at,
        };
        previous.push(record.clone());

        if modified_indices.contains(&i) {
            // Modify the file (change size or mtime)
            current.push(FileRecord {
                path: path.clone(),
                absolute_path: path.clone(),
                size: size + 1,               // Change size
                modified_at: modified_at + 1, // Change mtime
            });
        } else {
            current.push(record);
        }
    }

    let changes = ChangeDetector::detect(&current, &previous);

    prop_assert_eq!(changes.modified.len(), expected_modified);
    prop_assert_eq!(changes.added.len(), 0);
    prop_assert_eq!(changes.deleted.len(), 0);
    Ok(())
}

fn prop_change_detector_deterministic(
    current: Vec<(String, u64, i64)>,
    previous: Vec<(String, u64, i64)>,
) -> TestCaseResult {
    let current_records: Vec<FileRecord> = current
        .iter()
        .map(|(path, size, modified_at)| FileRecord {
            path: path.clone(),
            absolute_path: path.clone(),
            size: *size,
            modified_at: *modified_at,
        })
        .collect();
    let previous_records: Vec<FileRecord> = previous
        .iter()
        .map(|(path, size, modified_at)| FileRecord {
            path: path.clone(),
            absolute_path: path.clone(),
            size: *size,
            modified_at: *modified_at,
        })
        .collect();

    let changes1 = ChangeDetector::detect(&current_records, &previous_records);
    let changes2 = ChangeDetector::detect(&current_records, &previous_records);

    prop_assert_eq!(changes1.added.len(), changes2.added.len());
    prop_assert_eq!(changes1.modified.len(), changes2.modified.len());
    prop_assert_eq!(changes1.deleted.len(), changes2.deleted.len());
    prop_assert_eq!(changes1.unchanged.len(), changes2.unchanged.len());
    Ok(())
}

fn prop_change_detector_empty_current(previous: Vec<(String, u64, i64)>) -> TestCaseResult {
    let previous_records: Vec<FileRecord> = previous
        .iter()
        .map(|(path, size, modified_at)| FileRecord {
            path: path.clone(),
            absolute_path: path.clone(),
            size: *size,
            modified_at: *modified_at,
        })
        .collect();

    let changes = ChangeDetector::detect(&[], &previous_records);

    prop_assert_eq!(changes.added.len(), 0);
    prop_assert_eq!(changes.modified.len(), 0);
    prop_assert_eq!(changes.deleted.len(), previous.len());
    prop_assert_eq!(changes.unchanged.len(), 0);
    Ok(())
}

fn prop_change_detector_empty_previous(current: Vec<(String, u64, i64)>) -> TestCaseResult {
    let current_records: Vec<FileRecord> = current
        .iter()
        .map(|(path, size, modified_at)| FileRecord {
            path: path.clone(),
            absolute_path: path.clone(),
            size: *size,
            modified_at: *modified_at,
        })
        .collect();

    let changes = ChangeDetector::detect(&current_records, &[]);

    prop_assert_eq!(changes.added.len(), current.len());
    prop_assert_eq!(changes.modified.len(), 0);
    prop_assert_eq!(changes.deleted.len(), 0);
    prop_assert_eq!(changes.unchanged.len(), 0);
    Ok(())
}

fn prop_change_detector_size_only_change(files: Vec<(String, u64, i64)>) -> TestCaseResult {
    if files.is_empty() {
        return Ok(());
    }

    let mut current = Vec::new();
    let mut previous = Vec::new();

    for (path, size, modified_at) in &files {
        previous.push(FileRecord {
            path: path.clone(),
            absolute_path: path.clone(),
            size: *size,
            modified_at: *modified_at,
        });
        current.push(FileRecord {
            path: path.clone(),
            absolute_path: path.clone(),
            size: *size + 100,         // Only size changes
            modified_at: *modified_at, // Same mtime
        });
    }

    let changes = ChangeDetector::detect(&current, &previous);

    prop_assert_eq!(changes.modified.len(), files.len());
    prop_assert_eq!(changes.added.len(), 0);
    prop_assert_eq!(changes.deleted.len(), 0);
    Ok(())
}

fn prop_change_detector_mtime_only_change(files: Vec<(String, u64, i64)>) -> TestCaseResult {
    if files.is_empty() {
        return Ok(());
    }

    let mut current = Vec::new();
    let mut previous = Vec::new();

    for (path, size, modified_at) in &files {
        previous.push(FileRecord {
            path: path.clone(),
            absolute_path: path.clone(),
            size: *size,
            modified_at: *modified_at,
        });
        current.push(FileRecord {
            path: path.clone(),
            absolute_path: path.clone(),
            size: *size,                      // Same size
            modified_at: *modified_at + 3600, // Only mtime changes (1 hour)
        });
    }

    let changes = ChangeDetector::detect(&current, &previous);

    prop_assert_eq!(changes.modified.len(), files.len());
    prop_assert_eq!(changes.added.len(), 0);
    prop_assert_eq!(changes.deleted.len(), 0);
    Ok(())
}

// Change detector property tests
proptest! {
    #[test]
    fn change_detector_identical_files_unchanged(
        files in prop::collection::vec(
            ("[a-z/]+\\.md", 1u64..10000u64, 1i64..1000000i64),
            0..20
        )
    ) {
        prop_change_detector_identical_files_unchanged(files)?;
    }

    #[test]
    fn change_detector_new_files_added(
        current_files in prop::collection::vec(
            ("[a-z/]+\\.md", 1u64..10000u64, 1i64..1000000i64),
            0..10
        ),
        new_files in prop::collection::vec(
            ("[a-z/]+\\.md", 1u64..10000u64, 1i64..1000000i64),
            0..10
        )
    ) {
        prop_change_detector_new_files_added(current_files, new_files)?;
    }

    #[test]
    fn change_detector_deleted_files(
        current_files in prop::collection::vec(
            ("[a-z/]+\\.md", 1u64..10000u64, 1i64..1000000i64),
            1..20
        ),
        deleted_count in 0..20usize
    ) {
        prop_change_detector_deleted_files(current_files, deleted_count)?;
    }

    #[test]
    fn change_detector_modified_files(
        files in prop::collection::vec(
            ("[a-z/]+\\.md", 1u64..10000u64, 1i64..1000000i64),
            1..20
        ),
        modified_indices in prop::collection::vec(0..20usize, 0..20)
    ) {
        prop_change_detector_modified_files(files, modified_indices)?;
    }

    #[test]
    fn change_detector_deterministic(
        current in prop::collection::vec(
            ("[a-z/]+\\.md", 1u64..10000u64, 1i64..1000000i64),
            0..20
        ),
        previous in prop::collection::vec(
            ("[a-z/]+\\.md", 1u64..10000u64, 1i64..1000000i64),
            0..20
        )
    ) {
        prop_change_detector_deterministic(current, previous)?;
    }

    #[test]
    fn change_detector_empty_current(
        previous in prop::collection::vec(
            ("[a-z/]+\\.md", 1u64..10000u64, 1i64..1000000i64),
            0..20
        )
    ) {
        prop_change_detector_empty_current(previous)?;
    }

    #[test]
    fn change_detector_empty_previous(
        current in prop::collection::vec(
            ("[a-z/]+\\.md", 1u64..10000u64, 1i64..1000000i64),
            0..20
        )
    ) {
        prop_change_detector_empty_previous(current)?;
    }

    #[test]
    fn change_detector_size_only_change(
        files in prop::collection::vec(
            ("[a-z/]+\\.md", 1u64..10000u64, 1i64..1000000i64),
            1..20
        )
    ) {
        prop_change_detector_size_only_change(files)?;
    }

    #[test]
    fn change_detector_mtime_only_change(
        files in prop::collection::vec(
            ("[a-z/]+\\.md", 1u64..10000u64, 1i64..1000000i64),
            1..20
        )
    ) {
        prop_change_detector_mtime_only_change(files)?;
    }
}
