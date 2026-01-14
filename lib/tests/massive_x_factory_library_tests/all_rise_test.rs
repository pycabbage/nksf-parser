use nksf_parser::{ParseError, parse_nksf};
use std::path::PathBuf;

fn get_fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("massive_x_factory_library_tests")
        .join("fixture")
        .join("All Rise.nksf")
}

#[test]
fn test_all_rise_parse_success() {
    let path = get_fixture_path();
    let result = parse_nksf(&path);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

#[test]
fn test_all_rise_metadata() {
    let path = get_fixture_path();
    let nksf = parse_nksf(&path).expect("Failed to parse");

    assert_eq!(nksf.metadata.name, "All Rise");
    assert_eq!(nksf.metadata.author, "Sami Rabia");
    assert_eq!(nksf.metadata.vendor, "Native Instruments");
    assert_eq!(nksf.metadata.device_type, "INST");
    assert!(!nksf.metadata.uuid.is_empty());
}

#[test]
fn test_all_rise_parameters() {
    let path = get_fixture_path();
    let nksf = parse_nksf(&path).expect("Failed to parse");

    assert_eq!(nksf.parameters.ni8.len(), 2);

    let total_params: usize = nksf
        .parameters
        .ni8
        .iter()
        .filter_map(|v| v.as_array())
        .map(|arr| arr.len())
        .sum();

    assert_eq!(total_params, 16);
}

#[test]
fn test_all_rise_complete_parse() {
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
