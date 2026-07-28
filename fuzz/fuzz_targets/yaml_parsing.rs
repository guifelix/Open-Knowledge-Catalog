// Fuzz target for YAML parsing
#![no_main]
use libfuzzer_sys::fuzz_target;
use okc::parser::yaml::YamlParser;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = YamlParser::parse(s, 8 * 1024 * 1024);
    }
});