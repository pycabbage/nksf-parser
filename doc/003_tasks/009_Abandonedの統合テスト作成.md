# 009: Abandoned.nksfの統合テスト作成

## 概要

最初の統合テストとして `lib/tests/massive_x_factory_library_tests/abandoned_test.rs` を実装する。

## 完了条件

- [x] `lib/tests/massive_x_factory_library_tests/abandoned_test.rs` が更新されている
- [x] Abandoned.nksfファイルが正常に解析できることを確認するテストがある
- [x] メタデータの主要フィールドを検証するテストがある
- [x] パラメータ配列が期待通りであることを確認するテストがある
- [x] `cargo test -p nksf-parser test_abandoned` が成功すること

## 実装ガイド

### テストの構成

```rust
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
fn test_abandoned_metadata() {
    let path = get_fixture_path();
    let nksf = parse_nksf(&path).expect("Failed to parse");

    // メタデータの検証
    assert_eq!(nksf.metadata.name, "Abandoned");
    assert_eq!(nksf.metadata.author, "Torsten Fassbender");
    assert_eq!(nksf.metadata.vendor, "Native Instruments");
    assert_eq!(nksf.metadata.device_type, "INST");
    assert!(nksf.metadata.comment.contains("Massive X Library"));
}

#[test]
fn test_abandoned_parameters() {
    let path = get_fixture_path();
    let nksf = parse_nksf(&path).expect("Failed to parse");

    // パラメータの検証
    // NICAチャンクの構造に応じて検証内容を調整
    // 例: パラメータ配列が空でないこと
    assert!(!nksf.parameters.ni8.is_empty(), "Parameters should not be empty");
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
```

## 注意点

- `env!("CARGO_MANIFEST_DIR")` を使用してfixtureパスを解決
- メタデータの値は実際のファイルから取得した値と一致させる
- NICAチャンクの構造が明確になったら、パラメータテストを拡充する
- 完全なバイト解析が行われていることを確認するテストを含める

---

## 実装メモ

[実装時に発見した事柄や改善点などを記載]
