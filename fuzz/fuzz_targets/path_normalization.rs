// Fuzz target for path normalization
#![no_main]
use libfuzzer_sys::fuzz_target;
use okc::parser::link_utils::normalize_path;
use std::path::Path;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let path = Path::new(s);
        let _ = normalize_path(path);
    }
});
