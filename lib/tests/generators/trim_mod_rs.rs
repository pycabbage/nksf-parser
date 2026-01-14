// mod.rsから最初の720行（期待値モジュール）のみを残すスクリプト
use std::path::PathBuf;

#[test]
#[ignore]
fn trim_mod_rs_to_720_lines() {
    let mod_rs_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/massive_x_factory_library_tests/mod.rs");

    let content = std::fs::read_to_string(&mod_rs_path).unwrap();
    let lines: Vec<&str> = content.lines().collect();

    // 最初の720行のみを保持
    let trimmed = lines.iter().take(720).map(|s| s.to_string()).collect::<Vec<_>>().join("\n") + "\n";

    std::fs::write(&mod_rs_path, trimmed).unwrap();

    println!("✓ Trimmed mod.rs to 720 lines (expected_data modules only)");
}
