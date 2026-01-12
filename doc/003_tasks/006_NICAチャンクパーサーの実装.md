# 006: NICAチャンクパーサーの実装

## 概要

`lib/src/nica_parser.rs` を作成し、NICAチャンクのMessagePackデータをデシリアライズする。

## 完了条件

- [ ] `lib/src/nica_parser.rs` が作成されている
- [ ] バージョン情報の読み取り機能が実装されている
- [ ] MessagePackデシリアライズ機能が実装されている
- [ ] `NicaData` および `Parameter` への変換機能が実装されている
- [ ] 全バイトを解析し、未解析のバイトがないことを検証する
- [ ] ドキュメントコメントが記述されている（日本語）
- [ ] ユニットテストが記述されている
- [ ] `cargo test -p nksf-parser` が成功すること
- [ ] `cargo clippy` で警告が出ないこと

## 実装ガイド

### 関数定義

```rust
use crate::error::{ParseError, Result};
use crate::types::{NicaData, Parameter};
use rmp_serde;

/// NICAチャンクデータを解析
///
/// # Arguments
/// * `data` - チャンクデータ（バージョン情報を含む）
///
/// # Returns
/// * `Result<NicaData>` - 解析されたパラメータデータ
pub fn parse_nica_chunk(data: &[u8]) -> Result<NicaData> {
    // バージョンの読み取り（最初の4バイト、リトルエンディアン）
    if data.len() < 4 {
        return Err(ParseError::InvalidNiks);
    }

    let version = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);

    // バージョン1のみサポート
    if version != 1 {
        return Err(ParseError::InvalidNiks);
    }

    // MessagePackデータのデシリアライズ
    let nica_data: NicaData = rmp_serde::from_slice(&data[4..])
        .map_err(ParseError::MessagePackError)?;

    Ok(nica_data)
}
```

## 注意点

- バージョン情報は4バイト、リトルエンディアン
- MessagePackデータはバージョン情報の後から始まる
- NICAチャンクには `ni8` キーで配列が格納されている
- 配列の第1要素はパラメータリスト、第2要素は追加データ（構造を調査）
- 現時点ではバージョン1のみサポート
- 将来的なバージョン変更に対応できる設計にする
- ユニットテストはファイル内に記述する（`#[cfg(test)]` モジュール）

## NicaData の定義（types.rsで定義）

```rust
/// NICAチャンクのパラメータデータ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NicaData {
    pub ni8: Vec<serde_json::Value>, // または [Vec<Parameter>, AdditionalData]
}
```

---

## 実装メモ

[実装時に発見した事柄や改善点などを記載]
