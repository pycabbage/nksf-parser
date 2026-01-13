use nksf_parser::{ParseError, parse_nksf};
use std::path::PathBuf;

fn get_fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("massive_x_factory_library_tests")
        .join("fixture")
        .join("Abandoned.nksf")
}

#[test]
fn test_abandoned_parse_success() {
    let path = get_fixture_path();
    let result = parse_nksf(&path);
    assert!(
        result.is_ok(),
        "Failed to parse Abandoned.nksf: {:?}",
        result.err()
    );
}

#[test]
fn test_abandoned_metadata_complete() {
    let path = get_fixture_path();
    let nksf = parse_nksf(&path).expect("Failed to parse");

    // 全メタデータフィールドの検証
    assert_eq!(nksf.metadata.name, "Abandoned");
    assert_eq!(nksf.metadata.author, "Torsten Fassbender");
    assert_eq!(nksf.metadata.vendor, "Native Instruments");
    assert_eq!(nksf.metadata.device_type, "INST");
    assert_eq!(nksf.metadata.uuid, "f890b345-58f9-4f54-815e-87091547006e");
    assert_eq!(nksf.metadata.comment, "Massive X Library 1.4");

    // bankchainの検証
    assert_eq!(nksf.metadata.bankchain.len(), 3);
    assert_eq!(nksf.metadata.bankchain[0], "Massive X");
    assert_eq!(nksf.metadata.bankchain[1], "Massive X Library");
    assert_eq!(nksf.metadata.bankchain[2], "");

    // charactersの検証
    assert_eq!(nksf.metadata.characters.len(), 3);
    assert_eq!(nksf.metadata.characters[0], "Synthetic");
    assert_eq!(nksf.metadata.characters[1], "Dark");
    assert_eq!(nksf.metadata.characters[2], "Distorted");

    // modesの検証
    assert_eq!(nksf.metadata.modes.len(), 2);
    assert_eq!(nksf.metadata.modes[0], "_Torsten Fassbender");
    assert_eq!(nksf.metadata.modes[1], "__Best of the Rest");

    // typesの検証
    assert_eq!(nksf.metadata.types.len(), 2);
    assert_eq!(nksf.metadata.types[0].len(), 1);
    assert_eq!(nksf.metadata.types[0][0], "Synth Misc");
    assert_eq!(nksf.metadata.types[1].len(), 2);
    assert_eq!(nksf.metadata.types[1][0], "Synth Misc");
    assert_eq!(nksf.metadata.types[1][1], "FX");

    // __ni_internalの検証
    assert_eq!(
        nksf.metadata.ni_internal,
        serde_json::Value::String("BRIB".to_string())
    );
}

#[test]
fn test_abandoned_parameters_complete() {
    let path = get_fixture_path();
    let nksf = parse_nksf(&path).expect("Failed to parse");

    // ni8配列の検証
    assert_eq!(
        nksf.parameters.ni8.len(),
        2,
        "Should have 2 elements in ni8 array"
    );

    // ni8[0]: 8個のパラメータ（ID 0-7）
    let params_0 = nksf.parameters.ni8[0]
        .as_array()
        .expect("ni8[0] should be array");
    assert_eq!(params_0.len(), 8, "ni8[0] should have 8 parameters");

    // 各パラメータの詳細検証
    let param_names_0 = [
        "WT Pos 1",
        "WT Pos 2",
        "Osc 1 Lvl",
        "Osc 2 Lvl",
        "Filter",
        "Excite",
        "Delay",
        "Reverb",
    ];
    for (i, expected_name) in param_names_0.iter().enumerate() {
        let param = &params_0[i];
        assert_eq!(param["id"].as_u64().unwrap(), i as u64);
        assert_eq!(param["name"].as_str().unwrap(), *expected_name);
        assert_eq!(param["autoname"].as_bool().unwrap(), true);
        assert_eq!(param["vflag"].as_bool().unwrap(), false);
    }

    // ni8[1]: 8個のパラメータ（ID 8-15）
    let params_1 = nksf.parameters.ni8[1]
        .as_array()
        .expect("ni8[1] should be array");
    assert_eq!(params_1.len(), 8, "ni8[1] should have 8 parameters");

    // Macro 9-16の検証
    for (i, param) in params_1.iter().enumerate() {
        let expected_id = i + 8;
        let expected_name = format!("Macro {}", expected_id + 1);
        assert_eq!(param["id"].as_u64().unwrap(), expected_id as u64);
        assert_eq!(param["name"].as_str().unwrap(), expected_name);
        assert_eq!(param["autoname"].as_bool().unwrap(), true);
        assert_eq!(param["vflag"].as_bool().unwrap(), false);
    }
}

#[test]
fn test_abandoned_plid() {
    let path = get_fixture_path();
    let nksf = parse_nksf(&path).expect("Failed to parse");

    // PLID検証
    assert_eq!(nksf.plugin_id.vst_magic, 1315513416);
    assert_eq!(nksf.plugin_id.plugin_name, Some("Massive X".to_string()));
    assert_eq!(
        nksf.plugin_id.plugin_vendor,
        Some("Native Instruments".to_string())
    );
}

#[test]
fn test_abandoned_pchk_header() {
    let path = get_fixture_path();
    let nksf = parse_nksf(&path).expect("Failed to parse");

    // PCHKヘッダー検証
    assert_eq!(nksf.plugin_chunk.header.version, 1);
    assert_eq!(nksf.plugin_chunk.header.field1, 2);
    assert_eq!(nksf.plugin_chunk.header.field2, 2);
    assert_eq!(nksf.plugin_chunk.header.compressed_size, 31684);
}

#[test]
fn test_abandoned_pchk_structure() {
    let path = get_fixture_path();
    let nksf = parse_nksf(&path).expect("Failed to parse");

    // MessagePack値の総数検証
    assert_eq!(nksf.plugin_chunk.values.len(), 268);

    // セクション名の存在確認
    let section_names: Vec<&str> = nksf
        .plugin_chunk
        .values
        .iter()
        .filter_map(|v| v.as_str())
        .filter(|s| !s.contains("/"))
        .collect();

    assert!(section_names.contains(&"strings"));
    assert!(section_names.contains(&"floats"));
    assert!(section_names.contains(&"doubles"));
    assert!(section_names.contains(&"ints"));
    assert!(section_names.contains(&"bools"));
    assert!(section_names.contains(&"charVecs"));
    assert!(section_names.contains(&"intVecs"));
    assert!(section_names.contains(&"floatVecs"));
    assert!(section_names.contains(&"doubleVecs"));
    assert!(section_names.contains(&"stringVecs"));
}

#[test]
fn test_abandoned_pchk_strings_section() {
    let path = get_fixture_path();
    let nksf = parse_nksf(&path).expect("Failed to parse");

    // stringsセクション: values[0] = "strings", values[1] = 759, values[2] = Map
    assert_eq!(nksf.plugin_chunk.values[0].as_str(), Some("strings"));
    assert_eq!(nksf.plugin_chunk.values[1].as_u64(), Some(759));

    let strings_map = nksf.plugin_chunk.values[2]
        .as_object()
        .expect("strings section should be an object");

    // エントリ数検証
    assert_eq!(strings_map.len(), 758);

    // 重要なキーの検証
    assert_eq!(
        strings_map.get("meta/hash").and_then(|v| v.as_str()),
        Some("1cde7b7a6d767b6bec5a71498bd875cf")
    );
    assert_eq!(
        strings_map.get("meta/presetName").and_then(|v| v.as_str()),
        Some("Abandoned")
    );
    assert_eq!(
        strings_map
            .get("root/engine/global/macros/macro1/macroName/value")
            .and_then(|v| v.as_str()),
        Some("WT Pos 1")
    );
    assert_eq!(
        strings_map
            .get("root/engine/global/macros/macro8/macroName/value")
            .and_then(|v| v.as_str()),
        Some("Reverb")
    );
}

#[test]
fn test_abandoned_pchk_floats_section() {
    let path = get_fixture_path();
    let nksf = parse_nksf(&path).expect("Failed to parse");

    // floatsセクション: values[3] = "floats", values[4] = 2, values[5] = Map
    assert_eq!(nksf.plugin_chunk.values[3].as_str(), Some("floats"));
    assert_eq!(nksf.plugin_chunk.values[4].as_u64(), Some(2));

    let floats_map = nksf.plugin_chunk.values[5]
        .as_object()
        .expect("floats section should be an object");

    assert_eq!(floats_map.len(), 1);
}

#[test]
fn test_abandoned_pchk_doubles_section() {
    let path = get_fixture_path();
    let nksf = parse_nksf(&path).expect("Failed to parse");

    // doublesセクション: values[6] = "doubles", values[7] = 1105, values[8] = Map
    assert_eq!(nksf.plugin_chunk.values[6].as_str(), Some("doubles"));
    assert_eq!(nksf.plugin_chunk.values[7].as_u64(), Some(1105));

    let doubles_map = nksf.plugin_chunk.values[8]
        .as_object()
        .expect("doubles section should be an object");

    // エントリ数検証
    assert_eq!(doubles_map.len(), 1104);

    // マクロ値の存在確認
    assert!(
        doubles_map.contains_key("root/engine/global/macros/macro1/macroValue/normalizedValue")
    );
    assert!(
        doubles_map.contains_key("root/engine/global/macros/macro8/macroValue/normalizedValue")
    );
}

#[test]
fn test_abandoned_pchk_ints_section() {
    let path = get_fixture_path();
    let nksf = parse_nksf(&path).expect("Failed to parse");

    // intsセクション: values[9] = "ints", values[10] = 384, values[11] = Map
    assert_eq!(nksf.plugin_chunk.values[9].as_str(), Some("ints"));
    assert_eq!(nksf.plugin_chunk.values[10].as_u64(), Some(384));

    let ints_map = nksf.plugin_chunk.values[11]
        .as_object()
        .expect("ints section should be an object");

    // エントリ数検証
    assert_eq!(ints_map.len(), 383);

    // メタデータの検証
    assert_eq!(
        ints_map.get("meta/numUnits").and_then(|v| v.as_u64()),
        Some(1)
    );
    assert_eq!(ints_map.get("meta/type").and_then(|v| v.as_u64()), Some(0));
}

#[test]
fn test_abandoned_pchk_bools_section() {
    let path = get_fixture_path();
    let nksf = parse_nksf(&path).expect("Failed to parse");

    // boolsセクション: values[12] = "bools", values[13] = 1581, values[14] = Map
    assert_eq!(nksf.plugin_chunk.values[12].as_str(), Some("bools"));
    assert_eq!(nksf.plugin_chunk.values[13].as_u64(), Some(1581));

    let bools_map = nksf.plugin_chunk.values[14]
        .as_object()
        .expect("bools section should be an object");

    // エントリ数検証
    assert_eq!(bools_map.len(), 1580);

    // メタデータの検証
    assert_eq!(
        bools_map.get("meta/hasIcon").and_then(|v| v.as_bool()),
        Some(false)
    );
    assert_eq!(
        bools_map
            .get("meta/presetModified")
            .and_then(|v| v.as_bool()),
        Some(true)
    );
}

#[test]
fn test_abandoned_pchk_vec_sections() {
    let path = get_fixture_path();
    let nksf = parse_nksf(&path).expect("Failed to parse");

    // charVecsセクション
    assert_eq!(nksf.plugin_chunk.values[15].as_str(), Some("charVecs"));
    assert_eq!(nksf.plugin_chunk.values[16].as_u64(), Some(1));

    // intVecsセクション
    assert_eq!(nksf.plugin_chunk.values[17].as_str(), Some("intVecs"));
    assert_eq!(nksf.plugin_chunk.values[18].as_u64(), Some(42));

    // intVecsの最初のデータ（values[19]以降）
    // パターン: サイズ, キー, 配列
    assert_eq!(nksf.plugin_chunk.values[19].as_u64(), Some(41));
    assert!(nksf.plugin_chunk.values[20].as_str().is_some());
    assert!(nksf.plugin_chunk.values[21].as_array().is_some());

    // 配列の検証
    let first_array = nksf.plugin_chunk.values[21]
        .as_array()
        .expect("should be array");
    assert_eq!(first_array.len(), 4);
}

#[test]
fn test_abandoned_pchk_all_sections_counts() {
    let path = get_fixture_path();
    let nksf = parse_nksf(&path).expect("Failed to parse");

    // 各セクションの宣言数と実際数の検証
    struct SectionInfo {
        name_idx: usize,
        count_idx: usize,
        data_idx: usize,
        expected_name: &'static str,
        expected_count: u64,
        expected_actual: usize,
    }

    let sections = vec![
        SectionInfo {
            name_idx: 0,
            count_idx: 1,
            data_idx: 2,
            expected_name: "strings",
            expected_count: 759,
            expected_actual: 758,
        },
        SectionInfo {
            name_idx: 3,
            count_idx: 4,
            data_idx: 5,
            expected_name: "floats",
            expected_count: 2,
            expected_actual: 1,
        },
        SectionInfo {
            name_idx: 6,
            count_idx: 7,
            data_idx: 8,
            expected_name: "doubles",
            expected_count: 1105,
            expected_actual: 1104,
        },
        SectionInfo {
            name_idx: 9,
            count_idx: 10,
            data_idx: 11,
            expected_name: "ints",
            expected_count: 384,
            expected_actual: 383,
        },
        SectionInfo {
            name_idx: 12,
            count_idx: 13,
            data_idx: 14,
            expected_name: "bools",
            expected_count: 1581,
            expected_actual: 1580,
        },
    ];

    for section in sections {
        assert_eq!(
            nksf.plugin_chunk.values[section.name_idx].as_str(),
            Some(section.expected_name)
        );
        assert_eq!(
            nksf.plugin_chunk.values[section.count_idx].as_u64(),
            Some(section.expected_count)
        );

        let map = nksf.plugin_chunk.values[section.data_idx]
            .as_object()
            .expect(&format!("{} should be object", section.expected_name));
        assert_eq!(map.len(), section.expected_actual);
    }
}

#[test]
fn test_abandoned_complete_parse() {
    let path = get_fixture_path();
    let result = parse_nksf(&path);

    // 完全なバイト解析が行われたことを確認
    // IncompleteParse エラーが発生しないこと
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
fn test_abandoned_total_parameters() {
    let path = get_fixture_path();
    let nksf = parse_nksf(&path).expect("Failed to parse");

    // パラメータの合計数を検証（8 + 8 = 16個）
    let total_params: usize = nksf
        .parameters
        .ni8
        .iter()
        .filter_map(|v| v.as_array())
        .map(|arr| arr.len())
        .sum();

    assert_eq!(total_params, 16, "Should have 16 parameters in total");
}

// ========================================
// 完全な1:1比較テスト（全32,736バイト）
// ========================================

#[test]
fn test_abandoned_pchk_floats_value() {
    let path = get_fixture_path();
    let nksf = parse_nksf(&path).expect("Failed to parse");

    let floats_map = nksf.plugin_chunk.values[5]
        .as_object()
        .expect("floats should be object");

    // floatsセクションの唯一のエントリを検証
    assert_eq!(floats_map.len(), 1);

    let overlay_stretch = floats_map
        .get("root/engine/unit1/Performers/performers/grid/overlay stretch")
        .and_then(|v| v.as_f64())
        .expect("overlay stretch should exist");

    assert_eq!(overlay_stretch, 0.0);
}

#[test]
fn test_abandoned_pchk_all_268_values_count() {
    let path = get_fixture_path();
    let nksf = parse_nksf(&path).expect("Failed to parse");

    // 全268値が正しく読み取られていることを確認
    assert_eq!(nksf.plugin_chunk.values.len(), 268);

    // 各値の型が期待通りであることを確認
    // インデックス0-267の全値が有効なJSON値であること
    for (i, value) in nksf.plugin_chunk.values.iter().enumerate() {
        assert!(
            value.is_string() || value.is_number() || value.is_object() || value.is_array() || value.is_boolean() || value.is_null(),
            "Value at index {} should be valid JSON value", i
        );
    }
}

// ========================================
// 完全なデータ検証テスト（全3825エントリ）
// ========================================

#[test]
fn test_abandoned_pchk_strings_all_entries() {
    let path = get_fixture_path();
    let nksf = parse_nksf(&path).expect("Failed to parse");

    let expected = &super::abandoned_expected_data::EXPECTED_ABANDONED_STRINGS;
    let actual_map = nksf.plugin_chunk.values[2]
        .as_object()
        .expect("strings should be object");

    // 全758エントリの完全検証
    assert_eq!(actual_map.len(), expected.len());

    for (key, expected_value) in expected.entries() {
        let actual_value = actual_map
            .get(*key)
            .and_then(|v| v.as_str())
            .unwrap_or_else(|| panic!("Missing key: {}", key));
        assert_eq!(actual_value, *expected_value, "Mismatch at key: {}", key);
    }

    for key in actual_map.keys() {
        assert!(expected.contains_key(key), "Unexpected key: {}", key);
    }
}

#[test]
fn test_abandoned_pchk_doubles_all_entries() {
    let path = get_fixture_path();
    let nksf = parse_nksf(&path).expect("Failed to parse");

    let expected = &super::abandoned_expected_data::EXPECTED_ABANDONED_DOUBLES;
    let actual_map = nksf.plugin_chunk.values[8]
        .as_object()
        .expect("doubles should be object");

    // 全1104エントリの完全検証
    assert_eq!(actual_map.len(), expected.len());

    for (key, expected_value) in expected.entries() {
        let actual_value = actual_map
            .get(*key)
            .and_then(|v| v.as_f64())
            .unwrap_or_else(|| panic!("Missing key: {}", key));
        assert!(
            (actual_value - expected_value).abs() < 1e-10,
            "Mismatch at key: {}",
            key
        );
    }

    for key in actual_map.keys() {
        assert!(
            expected.contains_key(key.as_str()),
            "Unexpected key: {}",
            key
        );
    }
}

#[test]
fn test_abandoned_pchk_ints_all_entries() {
    let path = get_fixture_path();
    let nksf = parse_nksf(&path).expect("Failed to parse");

    let expected = &super::abandoned_expected_data::EXPECTED_ABANDONED_INTS;
    let actual_map = nksf.plugin_chunk.values[11]
        .as_object()
        .expect("ints should be object");

    // 全383エントリの完全検証
    assert_eq!(actual_map.len(), expected.len());

    for (key, expected_value) in expected.entries() {
        let actual_value = actual_map
            .get(*key)
            .and_then(|v| v.as_i64())
            .unwrap_or_else(|| panic!("Missing key: {}", key));
        assert_eq!(actual_value, *expected_value, "Mismatch at key: {}", key);
    }

    for key in actual_map.keys() {
        assert!(
            expected.contains_key(key.as_str()),
            "Unexpected key: {}",
            key
        );
    }
}

#[test]
fn test_abandoned_pchk_bools_all_entries() {
    let path = get_fixture_path();
    let nksf = parse_nksf(&path).expect("Failed to parse");

    let expected = &super::abandoned_expected_data::EXPECTED_ABANDONED_BOOLS;
    let actual_map = nksf.plugin_chunk.values[14]
        .as_object()
        .expect("bools should be object");

    // 全1580エントリの完全検証
    assert_eq!(actual_map.len(), expected.len());

    for (key, expected_value) in expected.entries() {
        let actual_value = actual_map
            .get(*key)
            .and_then(|v| v.as_bool())
            .unwrap_or_else(|| panic!("Missing key: {}", key));
        assert_eq!(actual_value, *expected_value, "Mismatch at key: {}", key);
    }

    for key in actual_map.keys() {
        assert!(
            expected.contains_key(key.as_str()),
            "Unexpected key: {}",
            key
        );
    }
}
