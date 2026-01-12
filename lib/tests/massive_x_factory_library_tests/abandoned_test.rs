use nksf_parser::{parse_nksf, ParseError};
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
    assert!(result.is_ok(), "Failed to parse Abandoned.nksf: {:?}", result.err());
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
    assert_eq!(nksf.metadata.ni_internal, serde_json::Value::String("BRIB".to_string()));
}

#[test]
fn test_abandoned_parameters_complete() {
    let path = get_fixture_path();
    let nksf = parse_nksf(&path).expect("Failed to parse");

    // ni8配列の検証
    assert_eq!(nksf.parameters.ni8.len(), 2, "Should have 2 elements in ni8 array");

    // ni8[0]: 8個のパラメータ（ID 0-7）
    let params_0 = nksf.parameters.ni8[0].as_array().expect("ni8[0] should be array");
    assert_eq!(params_0.len(), 8, "ni8[0] should have 8 parameters");

    // 各パラメータの詳細検証
    let param_names_0 = ["WT Pos 1", "WT Pos 2", "Osc 1 Lvl", "Osc 2 Lvl", "Filter", "Excite", "Delay", "Reverb"];
    for (i, expected_name) in param_names_0.iter().enumerate() {
        let param = &params_0[i];
        assert_eq!(param["id"].as_u64().unwrap(), i as u64);
        assert_eq!(param["name"].as_str().unwrap(), *expected_name);
        assert_eq!(param["autoname"].as_bool().unwrap(), true);
        assert_eq!(param["vflag"].as_bool().unwrap(), false);
    }

    // ni8[1]: 8個のパラメータ（ID 8-15）
    let params_1 = nksf.parameters.ni8[1].as_array().expect("ni8[1] should be array");
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
fn test_abandoned_unknown_chunks() {
    let path = get_fixture_path();
    let nksf = parse_nksf(&path).expect("Failed to parse");

    // 未知のチャンクの検証
    assert_eq!(nksf.unknown_chunks.len(), 2, "Should have 2 unknown chunks");

    // PLIDチャンク
    let plid = &nksf.unknown_chunks[0];
    assert_eq!(plid.id, "PLID");
    assert_eq!(plid.data.len(), 73, "PLID chunk should be 73 bytes");

    // PCHKチャンク
    let pchk = &nksf.unknown_chunks[1];
    assert_eq!(pchk.id, "PCHK");
    assert_eq!(pchk.data.len(), 31700, "PCHK chunk should be 31700 bytes");
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
            panic!("Incomplete parse: {} bytes remaining at offset {}", remaining, offset);
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
    let total_params: usize = nksf.parameters.ni8.iter()
        .filter_map(|v| v.as_array())
        .map(|arr| arr.len())
        .sum();

    assert_eq!(total_params, 16, "Should have 16 parameters in total");
}
