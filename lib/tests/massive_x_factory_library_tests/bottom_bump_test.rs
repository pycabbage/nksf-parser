// Bottom Bump
// 自動生成されたテストファイル

use nksf_parser::{ParseError, parse_nksf};
use std::path::PathBuf;

use super::bottom_bump_expected_data as expected;

fn get_fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("massive_x_factory_library_tests")
        .join("fixture")
        .join("Bottom Bump.nksf")
}

#[test]
fn test_bottom_bump_parse_success() {
    let path = get_fixture_path();
    let result = parse_nksf(&path);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

#[test]
fn test_bottom_bump_complete_parse() {
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
fn test_bottom_bump_nisi() {
    let path = get_fixture_path();
    let nksf = parse_nksf(&path).expect("Failed to parse");
    assert_eq!(nksf.metadata.name, expected::EXPECTED_NISI.name);
    assert_eq!(nksf.metadata.author, expected::EXPECTED_NISI.author);
    assert_eq!(nksf.metadata.vendor, expected::EXPECTED_NISI.vendor);
}
