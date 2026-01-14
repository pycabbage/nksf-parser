// 全720プリセット用のテストファイル生成スクリプト
use std::path::PathBuf;

#[test]
#[ignore]
fn generate_all_preset_test_files() {
    let fixture_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("massive_x_factory_library_tests")
        .join("fixture");

    let output_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("massive_x_factory_library_tests");

    let mut generated_count = 0;
    let mut module_names = Vec::new();

    println!("=== Generating Test Files ===\n");

    for entry in std::fs::read_dir(&fixture_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) != Some("nksf") {
            continue;
        }

        let file_name = path.file_stem().unwrap().to_str().unwrap();
        let safe_name = sanitize_module_name(file_name);

        // テストファイルを生成
        let test_code = generate_test_file_code(file_name, &safe_name);
        let output_path = output_dir.join(format!("{}_test.rs", safe_name));

        std::fs::write(&output_path, test_code).unwrap();

        module_names.push(safe_name);
        generated_count += 1;

        if generated_count % 100 == 0 {
            println!("Generated: {} test files", generated_count);
        }
    }

    println!("\n=== Generation Complete ===");
    println!("Test files: {}", generated_count);

    // mod.rsに追加
    let mut mod_additions = String::new();
    module_names.sort();
    for module_name in &module_names {
        mod_additions.push_str(&format!("mod {}_test;\n", module_name));
    }

    // 既存のmod.rsに追加
    let mod_rs_path = output_dir.join("mod.rs");
    let existing_content = std::fs::read_to_string(&mod_rs_path).unwrap();
    let new_content = existing_content + &mod_additions;
    std::fs::write(&mod_rs_path, new_content).unwrap();

    println!(
        "\n✓ Updated mod.rs with {} test modules",
        module_names.len()
    );
}

fn sanitize_module_name(name: &str) -> String {
    name.to_lowercase()
        .replace(" ", "_")
        .replace(".", "")
        .replace("-", "_")
        .replace("'", "")
        .replace("(", "")
        .replace(")", "")
        .replace("&", "and")
}

fn generate_test_file_code(file_name: &str, safe_name: &str) -> String {
    let mut code = String::new();

    code.push_str(&format!("// {}\n", file_name));
    code.push_str(&format!("// 自動生成されたテストファイル\n\n"));

    code.push_str("use nksf_parser::{parse_nksf, ParseError};\n");
    code.push_str("use std::path::PathBuf;\n\n");

    code.push_str(&format!(
        "use super::{}_expected_data as expected;\n\n",
        safe_name
    ));

    code.push_str("fn get_fixture_path() -> PathBuf {\n");
    code.push_str("    PathBuf::from(env!(\"CARGO_MANIFEST_DIR\"))\n");
    code.push_str("        .join(\"tests\")\n");
    code.push_str("        .join(\"massive_x_factory_library_tests\")\n");
    code.push_str("        .join(\"fixture\")\n");
    code.push_str(&format!(
        "        .join({:?})\n",
        format!("{}.nksf", file_name)
    ));
    code.push_str("}\n\n");

    // パース成功テスト
    code.push_str(&format!(
        "#[test]\nfn test_{}_parse_success() {{\n",
        safe_name
    ));
    code.push_str("    let path = get_fixture_path();\n");
    code.push_str("    let result = parse_nksf(&path);\n");
    code.push_str("    assert!(result.is_ok(), \"Failed to parse: {:?}\", result.err());\n");
    code.push_str("}\n\n");

    // 完全パーステスト
    code.push_str(&format!(
        "#[test]\nfn test_{}_complete_parse() {{\n",
        safe_name
    ));
    code.push_str("    let path = get_fixture_path();\n");
    code.push_str("    let result = parse_nksf(&path);\n");
    code.push_str("    match result {\n");
    code.push_str("        Ok(_) => { /* OK */ }\n");
    code.push_str("        Err(ParseError::IncompleteParse(remaining, offset)) => {\n");
    code.push_str("            panic!(\"Incomplete parse: {} bytes remaining at offset {}\", remaining, offset);\n");
    code.push_str("        }\n");
    code.push_str("        Err(e) => { panic!(\"Unexpected error: {:?}\", e); }\n");
    code.push_str("    }\n");
    code.push_str("}\n\n");

    // NISIテスト
    code.push_str(&format!("#[test]\nfn test_{}_nisi() {{\n", safe_name));
    code.push_str("    let path = get_fixture_path();\n");
    code.push_str("    let nksf = parse_nksf(&path).expect(\"Failed to parse\");\n");
    code.push_str("    assert_eq!(nksf.metadata.name, expected::EXPECTED_NISI.name);\n");
    code.push_str("    assert_eq!(nksf.metadata.author, expected::EXPECTED_NISI.author);\n");
    code.push_str("    assert_eq!(nksf.metadata.vendor, expected::EXPECTED_NISI.vendor);\n");
    code.push_str("}\n");

    code
}
