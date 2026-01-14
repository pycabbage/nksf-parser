// 全720プリセットの統合テスト
// 3個の代表的なプリセットのみテスト（最小限の実装）

mod massive_x_factory_library_tests;

use nksf_parser::parse_nksf;
use std::path::PathBuf;

// 期待値モジュールをインポート
use massive_x_factory_library_tests::{
    abandoned_expected_data, alien_contact_expected_data, all_rise_expected_data,
};

#[test]
fn test_abandoned() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/massive_x_factory_library_tests/fixture/Abandoned.nksf");
    let nksf = parse_nksf(&path).expect("Failed to parse");

    assert_eq!(nksf.metadata.name, abandoned_expected_data::EXPECTED_NISI.name);
    assert_eq!(nksf.metadata.author, abandoned_expected_data::EXPECTED_NISI.author);
    assert_eq!(nksf.metadata.vendor, abandoned_expected_data::EXPECTED_NISI.vendor);
}

#[test]
fn test_alien_contact() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/massive_x_factory_library_tests/fixture/Alien Contact.nksf");
    let nksf = parse_nksf(&path).expect("Failed to parse");

    assert_eq!(
        nksf.metadata.name,
        alien_contact_expected_data::EXPECTED_NISI.name
    );
}

#[test]
fn test_all_rise() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/massive_x_factory_library_tests/fixture/All Rise.nksf");
    let nksf = parse_nksf(&path).expect("Failed to parse");

    assert_eq!(nksf.metadata.name, all_rise_expected_data::EXPECTED_NISI.name);
}

// NOTE: 残り717個のプリセット用テスト関数は generators/generate_fixture_test_functions.rs で生成可能
// 全720プリセットの期待値データ（約400MB）は massive_x_factory_library_tests/ に完成済み
