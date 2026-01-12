# 005: NISIチャンクパーサーの実装

## 概要

`lib/src/nisi_parser.rs` を作成し、NISIチャンクのMessagePackデータをデシリアライズする。

## 完了条件

- [x] `lib/src/nisi_parser.rs` が作成されている
- [x] バージョン情報の読み取り機能が実装されている
- [x] MessagePackデシリアライズ機能が実装されている
- [x] `NisiMetadata` への変換機能が実装されている
- [x] 全バイトを解析し、未解析のバイトがないことを検証する
- [x] ドキュメントコメントが記述されている（日本語）
- [x] ユニットテストが記述されている
- [x] `cargo test -p nksf-parser` が成功すること
- [x] `cargo clippy` で警告が出ないこと

## 実装ガイド

### 関数定義

```rust
use crate::error::{ParseError, Result};
use crate::types::NisiMetadata;
use rmp_serde;

/// NISIチャンクデータを解析
///
/// # Arguments
/// * `data` - チャンクデータ（バージョン情報を含む）
///
/// # Returns
/// * `Result<NisiMetadata>` - 解析されたメタデータ
pub fn parse_nisi_chunk(data: &[u8]) -> Result<NisiMetadata> {
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
    let metadata: NisiMetadata = rmp_serde::from_slice(&data[4..])
        .map_err(ParseError::MessagePackError)?;

    Ok(metadata)
}
```

## 注意点

- バージョン情報は4バイト、リトルエンディアン
- MessagePackデータはバージョン情報の後から始まる
- 現時点ではバージョン1のみサポート
- 将来的なバージョン変更に対応できる設計にする
- エラーメッセージは明確にする
- ユニットテストはファイル内に記述する（`#[cfg(test)]` モジュール）
- テストでは実際のAbandoned.nksfから抽出したNISIチャンクデータを使用する

## テストデータの準備

```bash
# Abandoned.nksfからNISIチャンクを抽出（手動で確認）
hexdump -C lib/tests/massive_x_factory_library_tests/fixture/Abandoned.nksf -s 12 -n 341
```

---

## 実装メモ

[実装時に発見した事柄や改善点などを記載]
