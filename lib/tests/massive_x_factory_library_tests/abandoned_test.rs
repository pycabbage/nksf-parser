use nksf_parser::{ParseError, parse_nksf};
use std::path::PathBuf;

use super::abandoned_expected_data as expected;

fn get_fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("massive_x_factory_library_tests")
        .join("fixture")
        .join("Abandoned.nksf")
}

// =====================================
// 基本テスト
// =====================================

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
fn test_abandoned_complete_parse() {
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

// =====================================
// NISIチャンク検証
// =====================================

#[test]
fn test_abandoned_nisi_metadata() {
    let path = get_fixture_path();
    let nksf = parse_nksf(&path).expect("Failed to parse");

    let exp = &expected::EXPECTED_NISI;

    assert_eq!(nksf.metadata.name, exp.name);
    assert_eq!(nksf.metadata.author, exp.author);
    assert_eq!(nksf.metadata.vendor, exp.vendor);
    assert_eq!(nksf.metadata.device_type, exp.device_type);
    assert_eq!(nksf.metadata.uuid, exp.uuid);
    assert_eq!(nksf.metadata.comment, exp.comment);

    assert_eq!(nksf.metadata.bankchain.len(), exp.bankchain.len());
    for (i, expected_val) in exp.bankchain.iter().enumerate() {
        assert_eq!(nksf.metadata.bankchain[i], *expected_val);
    }

    assert_eq!(nksf.metadata.characters.len(), exp.characters.len());
    for (i, expected_val) in exp.characters.iter().enumerate() {
        assert_eq!(nksf.metadata.characters[i], *expected_val);
    }

    assert_eq!(nksf.metadata.modes.len(), exp.modes.len());
    for (i, expected_val) in exp.modes.iter().enumerate() {
        assert_eq!(nksf.metadata.modes[i], *expected_val);
    }

    assert_eq!(nksf.metadata.types.len(), exp.types.len());
    for (i, expected_arr) in exp.types.iter().enumerate() {
        assert_eq!(nksf.metadata.types[i].len(), expected_arr.len());
        for (j, expected_val) in expected_arr.iter().enumerate() {
            assert_eq!(nksf.metadata.types[i][j], *expected_val);
        }
    }

    assert_eq!(
        nksf.metadata.ni_internal,
        serde_json::Value::String(exp.ni_internal.to_string())
    );
}

// =====================================
// NICAチャンク検証
// =====================================

#[test]
fn test_abandoned_nica_parameters_0() {
    let path = get_fixture_path();
    let nksf = parse_nksf(&path).expect("Failed to parse");

    let params = nksf.parameters.ni8[0]
        .as_array()
        .expect("ni8[0] should be array");
    let expected = expected::EXPECTED_NICA_PARAMS_0;

    assert_eq!(params.len(), expected.len());

    for (i, exp) in expected.iter().enumerate() {
        let param = &params[i];
        assert_eq!(
            param["id"].as_u64().unwrap(),
            exp.id,
            "Param {} id mismatch",
            i
        );
        assert_eq!(
            param["name"].as_str().unwrap(),
            exp.name,
            "Param {} name mismatch",
            i
        );
        assert_eq!(
            param["autoname"].as_bool().unwrap(),
            exp.autoname,
            "Param {} autoname mismatch",
            i
        );
        assert_eq!(
            param["vflag"].as_bool().unwrap(),
            exp.vflag,
            "Param {} vflag mismatch",
            i
        );
    }
}

#[test]
fn test_abandoned_nica_parameters_1() {
    let path = get_fixture_path();
    let nksf = parse_nksf(&path).expect("Failed to parse");

    let params = nksf.parameters.ni8[1]
        .as_array()
        .expect("ni8[1] should be array");
    let expected = expected::EXPECTED_NICA_PARAMS_1;

    assert_eq!(params.len(), expected.len());

    for (i, exp) in expected.iter().enumerate() {
        let param = &params[i];
        assert_eq!(param["id"].as_u64().unwrap(), exp.id);
        assert_eq!(param["name"].as_str().unwrap(), exp.name);
        assert_eq!(param["autoname"].as_bool().unwrap(), exp.autoname);
        assert_eq!(param["vflag"].as_bool().unwrap(), exp.vflag);
    }
}

#[test]
fn test_abandoned_nica_total_parameters() {
    let path = get_fixture_path();
    let nksf = parse_nksf(&path).expect("Failed to parse");

    let total: usize = nksf
        .parameters
        .ni8
        .iter()
        .filter_map(|v| v.as_array())
        .map(|arr| arr.len())
        .sum();

    assert_eq!(total, 16);
}

// =====================================
// PLIDチャンク検証
// =====================================

#[test]
fn test_abandoned_plid() {
    let path = get_fixture_path();
    let nksf = parse_nksf(&path).expect("Failed to parse");

    assert_eq!(nksf.plugin_id.vst_magic, expected::EXPECTED_PLID_VST_MAGIC);
    assert_eq!(
        nksf.plugin_id.plugin_name.as_deref(),
        expected::EXPECTED_PLID_PLUGIN_NAME
    );
    assert_eq!(
        nksf.plugin_id.plugin_vendor.as_deref(),
        expected::EXPECTED_PLID_PLUGIN_VENDOR
    );
}

// =====================================
// PCHKチャンク - 構造検証
// =====================================

#[test]
fn test_abandoned_pchk_header() {
    let path = get_fixture_path();
    let nksf = parse_nksf(&path).expect("Failed to parse");

    assert_eq!(
        nksf.plugin_chunk.header.version,
        expected::EXPECTED_PCHK_VERSION
    );
    assert_eq!(
        nksf.plugin_chunk.header.field1,
        expected::EXPECTED_PCHK_FIELD1
    );
    assert_eq!(
        nksf.plugin_chunk.header.field2,
        expected::EXPECTED_PCHK_FIELD2
    );
    assert_eq!(
        nksf.plugin_chunk.header.compressed_size,
        expected::EXPECTED_PCHK_COMPRESSED_SIZE
    );
}

#[test]
fn test_abandoned_pchk_values_count() {
    let path = get_fixture_path();
    let nksf = parse_nksf(&path).expect("Failed to parse");

    assert_eq!(
        nksf.plugin_chunk.values.len(),
        expected::EXPECTED_PCHK_VALUES_COUNT
    );

    // 全268値が有効なJSON値であること
    for (i, value) in nksf.plugin_chunk.values.iter().enumerate() {
        assert!(
            value.is_string()
                || value.is_number()
                || value.is_object()
                || value.is_array()
                || value.is_boolean()
                || value.is_null(),
            "Value at index {} should be valid JSON value",
            i
        );
    }
}

#[test]
fn test_abandoned_pchk_section_structure() {
    let path = get_fixture_path();
    let nksf = parse_nksf(&path).expect("Failed to parse");

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

// =====================================
// PCHKチャンク - 個別セクション構造検証
// =====================================

#[test]
fn test_abandoned_pchk_section_counts() {
    let path = get_fixture_path();
    let nksf = parse_nksf(&path).expect("Failed to parse");

    struct Section {
        name_idx: usize,
        count_idx: usize,
        data_idx: usize,
        name: &'static str,
        declared: u64,
        actual: usize,
    }

    let sections = [
        Section {
            name_idx: 0,
            count_idx: 1,
            data_idx: 2,
            name: "strings",
            declared: 759,
            actual: 758,
        },
        Section {
            name_idx: 3,
            count_idx: 4,
            data_idx: 5,
            name: "floats",
            declared: 2,
            actual: 1,
        },
        Section {
            name_idx: 6,
            count_idx: 7,
            data_idx: 8,
            name: "doubles",
            declared: 1105,
            actual: 1104,
        },
        Section {
            name_idx: 9,
            count_idx: 10,
            data_idx: 11,
            name: "ints",
            declared: 384,
            actual: 383,
        },
        Section {
            name_idx: 12,
            count_idx: 13,
            data_idx: 14,
            name: "bools",
            declared: 1581,
            actual: 1580,
        },
    ];

    for sec in &sections {
        assert_eq!(
            nksf.plugin_chunk.values[sec.name_idx].as_str(),
            Some(sec.name)
        );
        assert_eq!(
            nksf.plugin_chunk.values[sec.count_idx].as_u64(),
            Some(sec.declared)
        );
        assert_eq!(
            nksf.plugin_chunk.values[sec.data_idx]
                .as_object()
                .unwrap()
                .len(),
            sec.actual
        );
    }
}

#[test]
fn test_abandoned_pchk_vec_sections() {
    let path = get_fixture_path();
    let nksf = parse_nksf(&path).expect("Failed to parse");

    // charVecs
    assert_eq!(nksf.plugin_chunk.values[15].as_str(), Some("charVecs"));
    assert_eq!(nksf.plugin_chunk.values[16].as_u64(), Some(1));

    // intVecs
    assert_eq!(nksf.plugin_chunk.values[17].as_str(), Some("intVecs"));
    assert_eq!(nksf.plugin_chunk.values[18].as_u64(), Some(42));

    // 最初のデータペア
    assert_eq!(nksf.plugin_chunk.values[19].as_u64(), Some(41));
    assert!(nksf.plugin_chunk.values[20].as_str().is_some());
    assert_eq!(nksf.plugin_chunk.values[21].as_array().unwrap().len(), 4);
}

// =====================================
// PCHKチャンク - 完全データ検証（全3826エントリ）
// =====================================

#[test]
fn test_abandoned_pchk_strings_all() {
    let path = get_fixture_path();
    let nksf = parse_nksf(&path).expect("Failed to parse");

    let expected_map = &expected::EXPECTED_ABANDONED_STRINGS;
    let actual_map = nksf.plugin_chunk.values[2].as_object().unwrap();

    assert_eq!(actual_map.len(), expected_map.len());

    for (key, expected_value) in expected_map.entries() {
        let actual_value = actual_map
            .get(*key)
            .and_then(|v| v.as_str())
            .unwrap_or_else(|| panic!("Missing key: {}", key));
        assert_eq!(actual_value, *expected_value, "Mismatch at key: {}", key);
    }

    for key in actual_map.keys() {
        assert!(expected_map.contains_key(key), "Unexpected key: {}", key);
    }
}

#[test]
fn test_abandoned_pchk_floats_value() {
    let path = get_fixture_path();
    let nksf = parse_nksf(&path).expect("Failed to parse");

    let floats_map = nksf.plugin_chunk.values[5].as_object().unwrap();
    let actual_val = floats_map
        .get("root/engine/unit1/Performers/performers/grid/overlay stretch")
        .and_then(|v| v.as_f64())
        .expect("overlay stretch should exist");

    assert_eq!(actual_val, expected::EXPECTED_FLOATS_OVERLAY_STRETCH);
}

#[test]
fn test_abandoned_pchk_doubles_all() {
    let path = get_fixture_path();
    let nksf = parse_nksf(&path).expect("Failed to parse");

    let expected_map = &expected::EXPECTED_ABANDONED_DOUBLES;
    let actual_map = nksf.plugin_chunk.values[8].as_object().unwrap();

    assert_eq!(actual_map.len(), expected_map.len());

    for (key, expected_value) in expected_map.entries() {
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
            expected_map.contains_key(key.as_str()),
            "Unexpected key: {}",
            key
        );
    }
}

#[test]
fn test_abandoned_pchk_ints_all() {
    let path = get_fixture_path();
    let nksf = parse_nksf(&path).expect("Failed to parse");

    let expected_map = &expected::EXPECTED_ABANDONED_INTS;
    let actual_map = nksf.plugin_chunk.values[11].as_object().unwrap();

    assert_eq!(actual_map.len(), expected_map.len());

    for (key, expected_value) in expected_map.entries() {
        let actual_value = actual_map
            .get(*key)
            .and_then(|v| v.as_i64())
            .unwrap_or_else(|| panic!("Missing key: {}", key));
        assert_eq!(actual_value, *expected_value, "Mismatch at key: {}", key);
    }

    for key in actual_map.keys() {
        assert!(
            expected_map.contains_key(key.as_str()),
            "Unexpected key: {}",
            key
        );
    }
}

#[test]
fn test_abandoned_pchk_bools_all() {
    let path = get_fixture_path();
    let nksf = parse_nksf(&path).expect("Failed to parse");

    let expected_map = &expected::EXPECTED_ABANDONED_BOOLS;
    let actual_map = nksf.plugin_chunk.values[14].as_object().unwrap();

    assert_eq!(actual_map.len(), expected_map.len());

    for (key, expected_value) in expected_map.entries() {
        let actual_value = actual_map
            .get(*key)
            .and_then(|v| v.as_bool())
            .unwrap_or_else(|| panic!("Missing key: {}", key));
        assert_eq!(actual_value, *expected_value, "Mismatch at key: {}", key);
    }

    for key in actual_map.keys() {
        assert!(
            expected_map.contains_key(key.as_str()),
            "Unexpected key: {}",
            key
        );
    }
}
