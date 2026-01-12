# 004: RIFFリーダーの実装

## 概要

`lib/src/riff_reader.rs` を作成し、RIFFファイルの読み取りとチャンク解析を実装する。

## 完了条件

- [ ] `lib/src/riff_reader.rs` が作成されている
- [ ] RIFFファイルのヘッダー検証機能が実装されている
- [ ] "NIKS" フォーマット識別子の検証機能が実装されている
- [ ] チャンクの列挙機能が実装されている
- [ ] チャンクデータの読み取り機能が実装されている
- [ ] 全バイトを追跡し、未読のバイトがないことを検証する機能がある
- [ ] ドキュメントコメントが記述されている（日本語）
- [ ] ユニットテストが記述されている
- [ ] `cargo test -p nksf-parser` が成功すること
- [ ] `cargo clippy` で警告が出ないこと

## 実装ガイド

### 構造体定義

```rust
use crate::error::{ParseError, Result};
use std::io::{Read, Seek};

/// RIFFチャンク情報
#[derive(Debug, Clone)]
pub struct RiffChunk {
    /// チャンクID（4文字）
    pub id: [u8; 4],
    /// チャンクサイズ（バイト）
    pub size: u32,
    /// チャンクデータの開始位置
    pub data_offset: u64,
}

/// RIFFリーダー
pub struct RiffReader<R: Read + Seek> {
    reader: R,
    file_size: u64,
    bytes_read: u64,
}
```

### 主要メソッド

```rust
impl<R: Read + Seek> RiffReader<R> {
    /// 新しいRIFFリーダーを作成
    pub fn new(mut reader: R) -> Result<Self> {
        // RIFFヘッダーの検証
        // "NIKS" フォーマット識別子の検証
    }

    /// 次のチャンクを読み取る
    pub fn next_chunk(&mut self) -> Result<Option<RiffChunk>> {
        // チャンク情報の読み取り
    }

    /// チャンクデータを読み取る
    pub fn read_chunk_data(&mut self, chunk: &RiffChunk) -> Result<Vec<u8>> {
        // チャンクデータの読み取り
    }

    /// 全バイトが読み取られたことを確認
    pub fn verify_complete(&self) -> Result<()> {
        if self.bytes_read < self.file_size {
            return Err(ParseError::IncompleteParse(
                (self.file_size - self.bytes_read) as usize,
                self.bytes_read as usize,
            ));
        }
        Ok(())
    }
}
```

## 注意点

- RIFFフォーマットはリトルエンディアン
- チャンクサイズにはチャンクID（4バイト）とサイズフィールド（4バイト）は含まれない
- 奇数バイトのチャンクには1バイトのパディングがある
- `bytes_read` で読み取ったバイト数を追跡し、完全性を検証する
- エラーハンドリングを適切に行う
- ユニットテストはファイル内に記述する（`#[cfg(test)]` モジュール）
- テストではモックデータを使用する

---

## 実装メモ

[実装時に発見した事柄や改善点などを記載]
