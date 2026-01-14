// mod.rsをpub modに変更するスクリプト
use std::path::PathBuf;

#[test]
#[ignore]
fn fix_mod_rs_to_pub() {
    let mod_rs_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("massive_x_factory_library_tests")
        .join("mod.rs");

    let content = std::fs::read_to_string(&mod_rs_path).unwrap();
    let new_content = content
        .lines()
        .map(|line| {
            if line.starts_with("mod ") && line.ends_with("_expected_data;") {
                line.replace("mod ", "pub mod ")
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";

    std::fs::write(&mod_rs_path, new_content).unwrap();

    println!("✓ Fixed mod.rs - all modules are now public");
}
