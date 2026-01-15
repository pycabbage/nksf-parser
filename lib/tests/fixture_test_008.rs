// プリセット統合テスト Part 8/8
// 自動生成：generators/generate_split_fixture_tests.rs

use nksf_parser::parse_nksf;
use rstest::rstest;
use std::path::PathBuf;

#[rstest]
#[case("Wide Awake")]
#[case("Wild Pitch")]
#[case("Wintery Tinkle")]
#[case("Wipeout")]
#[case("Wisdom Oracle")]
#[case("Wob Repeat")]
#[case("Wob You")]
#[case("Wobbly Orbit")]
#[case("Wonky Comb")]
#[case("Wooden Attack")]
#[case("Wooden Strike")]
#[case("Wooden String")]
#[case("Woozle")]
#[case("World Park")]
#[case("Wowzer")]
#[case("Wub Goo")]
#[case("Xberg Wunder")]
#[case("Zaddy")]
#[case("Zitteraal")]
#[case("Zytrus Silk")]
fn test_parse_preset(#[case] preset_name: &str) {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/massive_x_factory_library_tests/fixture")
        .join(format!("{}.nksf", preset_name));

    let nksf = parse_nksf(&path).expect("Failed to parse preset");

    // 全データをYAMLスナップショットとして検証
    insta::assert_yaml_snapshot!(preset_name, nksf);
}
