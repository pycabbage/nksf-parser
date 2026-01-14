// Seismic Freq
// 自動生成されたテストファイル

use nksf_parser::{ParseError, parse_nksf};
use std::path::PathBuf;

use super::seismic_freq_expected_data as expected;

fn get_fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("massive_x_factory_library_tests")
        .join("fixture")
        .join("Seismic Freq.nksf")
}

#[test]
fn test_seismic_freq_parse_success() {
    let path = get_fixture_path();
    let result = parse_nksf(&path);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

#[test]
fn test_seismic_freq_complete_parse() {
    let path = get_fixture_path();
    let result = parse_nksf(&path);
    match result {
        Ok(_) => { /* OK */ }
        Err(ParseError::IncompleteParse(remaining, offset)) => {
            panic!(
                "Incomplete parse: {} bytes remaining at offset {}",
                remaining, offset
            );
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

#[test]
fn test_seismic_freq_nisi() {
    let path = get_fixture_path();
    let nksf = parse_nksf(&path).expect("Failed to parse");
    assert_eq!(nksf.metadata.name, expected::EXPECTED_NISI.name);
    assert_eq!(nksf.metadata.author, expected::EXPECTED_NISI.author);
    assert_eq!(nksf.metadata.vendor, expected::EXPECTED_NISI.vendor);
}
