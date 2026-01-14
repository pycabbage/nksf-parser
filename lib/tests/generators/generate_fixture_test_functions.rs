// fixture_test.rs内に全720個のテスト関数を生成するスクリプト
use std::path::PathBuf;

#[test]
#[ignore]
fn generate_fixture_test_functions() {
    let fixture_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/massive_x_factory_library_tests/fixture");

    let mut test_functions = String::new();

    let mut preset_names = Vec::new();

    for entry in std::fs::read_dir(&fixture_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) != Some("nksf") {
            continue;
        }

        let file_name = path.file_stem().unwrap().to_str().unwrap();
        preset_names.push(file_name.to_string());
    }

    preset_names.sort();

    for file_name in &preset_names {
        let safe_name = sanitize_name(file_name);

        test_functions.push_str(&format!("\n#[test]\n"));
        test_functions.push_str(&format!("fn test_{}() {{\n", safe_name));
        test_functions.push_str(&format!("    let path = PathBuf::from(env!(\"CARGO_MANIFEST_DIR\"))\n"));
        test_functions.push_str(&format!("        .join(\"tests/massive_x_factory_library_tests/fixture/{}.nksf\");\n", file_name));
        test_functions.push_str(&format!("    let nksf = parse_nksf(&path).expect(\"Failed to parse\");\n\n"));
        test_functions.push_str(&format!("    assert_eq!(nksf.metadata.name, {}_expected_data::EXPECTED_NISI.name);\n", safe_name));
        test_functions.push_str(&format!("    assert_eq!(nksf.metadata.author, {}_expected_data::EXPECTED_NISI.author);\n", safe_name));
        test_functions.push_str(&format!("    assert_eq!(nksf.metadata.vendor, {}_expected_data::EXPECTED_NISI.vendor);\n", safe_name));
        test_functions.push_str(&format!("}}\n"));
    }

    // fixture_test.rsの完全版を生成
    let mut code = String::new();
    code.push_str("// 全720プリセットの統合テスト\n");
    code.push_str("// 自動生成：generators/generate_fixture_test_functions.rs\n\n");
    code.push_str("use nksf_parser::{parse_nksf, ParseError};\n");
    code.push_str("use std::path::PathBuf;\n\n");
    code.push_str("use crate::massive_x_factory_library_tests::*;\n\n");

    code.push_str(&test_functions);

    let output_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixture_test.rs");

    std::fs::write(&output_path, code).unwrap();

    println!("✓ Generated fixture_test.rs with {} test functions", preset_names.len());
}

fn sanitize_name(name: &str) -> String {
    name.to_lowercase()
        .replace(" ", "_")
        .replace(".", "")
        .replace("-", "_")
        .replace("'", "")
        .replace("(", "")
        .replace(")", "")
        .replace("&", "and")
}
