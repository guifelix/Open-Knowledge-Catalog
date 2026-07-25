// Fuzz target for frontmatter extraction
#![no_main]
use libfuzzer_sys::fuzz_target;
use okc::parser::frontmatter::FrontMatterExtractor;

fuzz_target!(|data: &[u8]| {
    let extractor = FrontMatterExtractor::new(1024 * 1024);
    let _ = extractor.extract(data);
    
    if data.len() > 10 {
        let small_extractor = FrontMatterExtractor::new(10);
        let _ = small_extractor.extract(data);
    }
});