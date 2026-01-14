// 全720プリセットの期待値ファイル生成スクリプト
use nksf_parser::parse_nksf;
use std::path::PathBuf;

#[test]
#[ignore] // 通常のテスト実行では除外（手動実行のみ）
fn generate_all_preset_expected_data() {
    let fixture_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("massive_x_factory_library_tests")
        .join("fixture");

    let output_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("massive_x_factory_library_tests");

    let mut generated_count = 0;
    let mut failed_count = 0;
    let mut module_names = Vec::new();

    println!("=== Generating Expected Data Files ===\n");

    for entry in std::fs::read_dir(&fixture_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) != Some("nksf") {
            continue;
        }

        let file_name = path.file_stem().unwrap().to_str().unwrap();

        // プリセットを解析
        let nksf = match parse_nksf(&path) {
            Ok(n) => n,
            Err(e) => {
                eprintln!("✗ Failed to parse {}: {}", file_name, e);
                failed_count += 1;
                continue;
            }
        };

        // ファイル名を安全な形式に変換
        let safe_name = sanitize_module_name(file_name);

        // 期待値ファイルを生成
        let expected_code = generate_expected_data_code(&nksf, file_name);
        let output_path = output_dir.join(format!("{}_expected_data.rs", safe_name));

        std::fs::write(&output_path, expected_code).unwrap();

        module_names.push(safe_name.clone());
        generated_count += 1;

        if generated_count % 50 == 0 {
            println!("Generated: {} files", generated_count);
        }
    }

    println!("\n=== Generation Complete ===");
    println!("Success: {} files", generated_count);
    println!("Failed: {} files", failed_count);

    // mod.rsファイルを生成
    let mut mod_rs_content = String::new();
    module_names.sort();
    for module_name in &module_names {
        mod_rs_content.push_str(&format!("pub mod {}_expected_data;\n", module_name));
    }

    let mod_rs_path = output_dir.join("mod.rs");
    std::fs::write(&mod_rs_path, mod_rs_content).unwrap();

    println!("\n✓ Generated mod.rs with {} modules", module_names.len());
}

fn sanitize_module_name(name: &str) -> String {
    name.to_lowercase()
        .replace(" ", "_")
        .replace(".", "")
        .replace("-", "_")
        .replace("'", "")
        .replace("(", "")
        .replace(")", "")
}

fn generate_expected_data_code(nksf: &nksf_parser::NksfFile, file_name: &str) -> String {
    let mut code = String::new();

    // ファイルヘッダー
    code.push_str(&format!("// 自動生成：{}の期待値データ\n\n", file_name));
    code.push_str("use phf::phf_map;\n\n");

    // NISI構造体定義
    code.push_str("pub struct ExpectedNisiMetadata {\n");
    code.push_str("    pub name: &'static str,\n");
    code.push_str("    pub author: &'static str,\n");
    code.push_str("    pub vendor: &'static str,\n");
    code.push_str("    pub device_type: &'static str,\n");
    code.push_str("    pub uuid: &'static str,\n");
    code.push_str("    pub comment: &'static str,\n");
    code.push_str("    pub bankchain: &'static [&'static str],\n");
    code.push_str("    pub characters: &'static [&'static str],\n");
    code.push_str("    pub modes: &'static [&'static str],\n");
    code.push_str("    pub types: &'static [&'static [&'static str]],\n");
    code.push_str("    pub ni_internal: &'static str,\n");
    code.push_str("}\n\n");

    // NISI期待値
    code.push_str("pub const EXPECTED_NISI: ExpectedNisiMetadata = ExpectedNisiMetadata {\n");
    code.push_str(&format!("    name: {:?},\n", nksf.metadata.name));
    code.push_str(&format!("    author: {:?},\n", nksf.metadata.author));
    code.push_str(&format!("    vendor: {:?},\n", nksf.metadata.vendor));
    code.push_str(&format!(
        "    device_type: {:?},\n",
        nksf.metadata.device_type
    ));
    code.push_str(&format!("    uuid: {:?},\n", nksf.metadata.uuid));
    code.push_str(&format!("    comment: {:?},\n", nksf.metadata.comment));

    // bankchain
    code.push_str("    bankchain: &[");
    for item in &nksf.metadata.bankchain {
        code.push_str(&format!("{:?}, ", item));
    }
    code.push_str("],\n");

    // characters
    code.push_str("    characters: &[");
    for item in &nksf.metadata.characters {
        code.push_str(&format!("{:?}, ", item));
    }
    code.push_str("],\n");

    // modes
    code.push_str("    modes: &[");
    for item in &nksf.metadata.modes {
        code.push_str(&format!("{:?}, ", item));
    }
    code.push_str("],\n");

    // types
    code.push_str("    types: &[");
    for types_arr in &nksf.metadata.types {
        code.push_str("&[");
        for item in types_arr {
            code.push_str(&format!("{:?}, ", item));
        }
        code.push_str("], ");
    }
    code.push_str("],\n");

    // ni_internal
    code.push_str(&format!(
        "    ni_internal: {:?},\n",
        nksf.metadata.ni_internal.as_str().unwrap_or("BRIB")
    ));
    code.push_str("};\n\n");

    // NICA期待値
    code.push_str("pub struct ExpectedParameter {\n");
    code.push_str("    pub id: u64,\n");
    code.push_str("    pub name: &'static str,\n");
    code.push_str("    pub autoname: bool,\n");
    code.push_str("    pub vflag: bool,\n");
    code.push_str("}\n\n");

    // NICA params 0
    code.push_str("pub const EXPECTED_NICA_PARAMS_0: &[ExpectedParameter] = &[\n");
    if !nksf.parameters.ni8.is_empty() {
        if let Some(arr) = nksf.parameters.ni8[0].as_array() {
            for param in arr {
                code.push_str("    ExpectedParameter {\n");
                code.push_str(&format!("        id: {},\n", param["id"].as_u64().unwrap()));
                code.push_str(&format!(
                    "        name: {:?},\n",
                    param["name"].as_str().unwrap()
                ));
                code.push_str(&format!(
                    "        autoname: {},\n",
                    param["autoname"].as_bool().unwrap()
                ));
                code.push_str(&format!(
                    "        vflag: {},\n",
                    param["vflag"].as_bool().unwrap()
                ));
                code.push_str("    },\n");
            }
        }
    }
    code.push_str("];\n\n");

    // NICA params 1
    code.push_str("pub const EXPECTED_NICA_PARAMS_1: &[ExpectedParameter] = &[\n");
    if nksf.parameters.ni8.len() > 1 {
        if let Some(arr) = nksf.parameters.ni8[1].as_array() {
            for param in arr {
                code.push_str("    ExpectedParameter {\n");
                code.push_str(&format!("        id: {},\n", param["id"].as_u64().unwrap()));
                code.push_str(&format!(
                    "        name: {:?},\n",
                    param["name"].as_str().unwrap()
                ));
                code.push_str(&format!(
                    "        autoname: {},\n",
                    param["autoname"].as_bool().unwrap()
                ));
                code.push_str(&format!(
                    "        vflag: {},\n",
                    param["vflag"].as_bool().unwrap()
                ));
                code.push_str("    },\n");
            }
        }
    }
    code.push_str("];\n\n");

    // PLID期待値
    code.push_str(&format!(
        "pub const EXPECTED_PLID_VST_MAGIC: u32 = {};\n",
        nksf.plugin_id.vst_magic
    ));
    code.push_str(&format!(
        "pub const EXPECTED_PLID_PLUGIN_NAME: Option<&'static str> = {:?};\n",
        nksf.plugin_id.plugin_name.as_deref()
    ));
    code.push_str(&format!(
        "pub const EXPECTED_PLID_PLUGIN_VENDOR: Option<&'static str> = {:?};\n\n",
        nksf.plugin_id.plugin_vendor.as_deref()
    ));

    // PCHKヘッダー期待値
    code.push_str(&format!(
        "pub const EXPECTED_PCHK_VERSION: u32 = {};\n",
        nksf.plugin_chunk.header.version
    ));
    code.push_str(&format!(
        "pub const EXPECTED_PCHK_FIELD1: u32 = {};\n",
        nksf.plugin_chunk.header.field1
    ));
    code.push_str(&format!(
        "pub const EXPECTED_PCHK_FIELD2: u32 = {};\n",
        nksf.plugin_chunk.header.field2
    ));
    code.push_str(&format!(
        "pub const EXPECTED_PCHK_COMPRESSED_SIZE: u32 = {};\n",
        nksf.plugin_chunk.header.compressed_size
    ));
    code.push_str(&format!(
        "pub const EXPECTED_PCHK_VALUES_COUNT: usize = {};\n\n",
        nksf.plugin_chunk.values.len()
    ));

    // PCHK sections

    // strings
    code.push_str(
        "pub static EXPECTED_ABANDONED_STRINGS: phf::Map<&'static str, &'static str> = phf_map! {\n",
    );
    if let Some(strings_map) = nksf.plugin_chunk.values[2].as_object() {
        for (key, value) in strings_map {
            let val_str = value
                .as_str()
                .unwrap()
                .replace("\\", "\\\\")
                .replace("\"", "\\\"")
                .replace("\n", "\\n")
                .replace("\r", "\\r")
                .replace("\t", "\\t");
            code.push_str(&format!(
                "    \"{}\" => \"{}\",\n",
                key.replace("\\", "\\\\").replace("\"", "\\\""),
                val_str
            ));
        }
    }
    code.push_str("};\n\n");

    // floats
    code.push_str("pub const EXPECTED_FLOATS_OVERLAY_STRETCH: f64 = ");
    if let Some(floats_map) = nksf.plugin_chunk.values[5].as_object() {
        let overlay_val = floats_map
            .get("root/engine/unit1/Performers/performers/grid/overlay stretch")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
        // f64として明示的にフォーマット（整数値でも .0 を付ける）
        let val_str = if overlay_val.fract() == 0.0 {
            format!("{:.1}", overlay_val)
        } else {
            format!("{}", overlay_val)
        };
        code.push_str(&format!("{};\n\n", val_str));
    } else {
        code.push_str("0.0;\n\n");
    }

    // doubles
    code.push_str(
        "pub static EXPECTED_ABANDONED_DOUBLES: phf::Map<&'static str, f64> = phf_map! {\n",
    );
    if let Some(doubles_map) = nksf.plugin_chunk.values[8].as_object() {
        for (key, value) in doubles_map {
            let val = value.as_f64().unwrap();
            let val_str = if val.fract() == 0.0 && val.abs() < 1e10 {
                format!("{:.1}", val)
            } else {
                format!("{}", val)
            };
            code.push_str(&format!(
                "    \"{}\" => {},\n",
                key.replace("\\", "\\\\").replace("\"", "\\\""),
                val_str
            ));
        }
    }
    code.push_str("};\n\n");

    // ints
    code.push_str("pub static EXPECTED_ABANDONED_INTS: phf::Map<&'static str, i64> = phf_map! {\n");
    if let Some(ints_map) = nksf.plugin_chunk.values[11].as_object() {
        for (key, value) in ints_map {
            let val = value.as_i64().unwrap();
            code.push_str(&format!(
                "    \"{}\" => {},\n",
                key.replace("\\", "\\\\").replace("\"", "\\\""),
                val
            ));
        }
    }
    code.push_str("};\n\n");

    // bools
    code.push_str(
        "pub static EXPECTED_ABANDONED_BOOLS: phf::Map<&'static str, bool> = phf_map! {\n",
    );
    if let Some(bools_map) = nksf.plugin_chunk.values[14].as_object() {
        for (key, value) in bools_map {
            let val = value.as_bool().unwrap();
            code.push_str(&format!(
                "    \"{}\" => {},\n",
                key.replace("\\", "\\\\").replace("\"", "\\\""),
                val
            ));
        }
    }
    code.push_str("};\n");

    code
}
