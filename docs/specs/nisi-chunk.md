# NISI チャンク（メタデータ）

## 概要

NISI チャンクは Massive X プリセットのメタデータを格納する。プリセット名、作者、ベンダー、バンクチェーン、タグ類、UUID など、ホスト (DAW) のブラウザ表示に使われる情報が含まれる。ペイロードは MessagePack 形式のマップであり、エンコーディングの基礎は [messagepack-encoding.md](messagepack-encoding.md) を参照。

> 実装: [`lib/src/nisi_parser.rs`](../../lib/src/nisi_parser.rs)（`parse_nisi_chunk`）、構造体は [`lib/src/types.rs`](../../lib/src/types.rs) の `NisiMetadata`

## チャンクレイアウト

`Abandoned.nksf` での観測値（チャンク全体で約 349 バイト）:

| Offset（チャンク先頭基準） | Size | Content | Description |
|--------|------|---------|-------------|
| +0x00 | 4 | `"NISI"` | チャンク ID（ASCII） |
| +0x04 | 4 | chunk_size | チャンクサイズ（リトルエンディアン u32）。観測例: 341 |
| +0x08 | 4 | version | チャンクバージョン（リトルエンディアン u32）。観測値: 1 |
| +0x0C | 約341 | MessagePack データ | メタデータマップ |

MessagePack 部分の先頭バイトは `0x8b`（要素数 11 の fixmap）で始まる。以降の説明では `Abandoned.nksf` のオフセット（ファイル先頭基準）を用いる。この場合チャンクデータ部（MessagePack 開始位置）はファイルオフセット `0x14` からである。

## MessagePack 構造

トップレベルは **fixmap（要素数 11、`0x8b`）** である。

### フィールド一覧

| キー | 値の型 | 説明 | Abandoned.nksf の観測値 |
|---|---|---|---|
| `__ni_internal` | マップ | 内部データ。キー `BRIB` を持つ | `{"BRIB": ...}` |
| `author` | 文字列 | 作者名 | `"Torsten Fassbender"` |
| `bankchain` | 配列 | バンクチェーン | `["Massive X", "Massive X Library", ""]` |
| `characters` | 配列 | キャラクタータグ | `["Synthetic", "Dark", "Distorted"]` |
| `comment` | 文字列 | コメント | `"Massive X Library 1.4"` |
| `deviceType` | 文字列 | デバイスタイプ | `"INST"` |
| `modes` | 配列 | モードタグ | `["_Torsten Fassbender", "__Best of the Rest"]` |
| `name` | 文字列 | プリセット名 | `"Abandoned"` |
| `types` | 配列（配列の配列） | 分類パス | `[["Synth Misc"], ["Synth Misc", "FX"]]` |
| `uuid` | str8 (36バイト) | UUID 文字列 | `"f890b345-58f9-4f54-815e-87091547006e"` |
| `vendor` | 文字列 | ベンダー名 | `"Native Instruments"` |

補足:

- `uuid` は他の短い文字列と異なり **str8（`0xd9` + 長さ）** でエンコードされる（36 バイトの UUID 文字列）。
- `types` は階層分類を表すため「配列の配列」となる（例: `["Synth Misc", "FX"]` は Synth Misc → FX というパス）。
- 実装上、`__ni_internal` / `characters` / `modes` は存在しないプリセットに備えてデフォルト値（Null / 空配列）でデシリアライズできるようになっている。
- `__ni_internal` の詳細構造は規定されていないため、実装では生データを保持する型（`serde_json::Value` エイリアス `NiInternal`）で受けている。

## 先頭バイトのデコード例

`Abandoned.nksf` のファイルオフセット `0x14` 以降（MessagePack データ部の先頭）:

```hex
Offset  Bytes                               Decoded
------  --------------------------------    ------------------------------
0x14    8b                                  fixmap (要素数 11)
0x15    ad                                  fixstr (13文字)
0x16    5f 5f 6e 69 5f 69 6e 74 65 72       "__ni_internal"
        6e 61 6c
0x22    a4                                  fixstr (4文字)
0x23    42 52 49 42                         "BRIB"
        (BRIB の値: 内部データが続く)
...     a6 61 75 74 68 6f 72                fixstr(6) "author"
        b2 54 6f 72 73 74 65 6e 20 46 61    fixstr(18) "Torsten Fassbender"
        73 73 62 65 6e 64 65 72
```

`"author"` の値 `"Torsten Fassbender"` は 18 文字のため fixstr の上限（31 文字）内であり、プレフィックス `0xb2 = 0xa0 + 18` 1 バイトのみで長さが表現される。

## Rust 構造体

実装では以下の構造体に直接デシリアライズする（`rmp-serde` 使用）。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NisiMetadata {
    #[serde(rename = "__ni_internal", default)]
    pub ni_internal: NiInternal,   // 詳細未規定の内部データ
    pub author: String,
    pub bankchain: Vec<String>,
    #[serde(default)]
    pub characters: Vec<String>,   // 古いプリセットには存在しないことがある
    pub comment: String,
    #[serde(rename = "deviceType")]
    pub device_type: String,
    #[serde(default)]
    pub modes: Vec<String>,
    pub name: String,
    pub types: Vec<Vec<String>>,
    pub uuid: String,
    pub vendor: String,
}
```

さらにパーサーは `name` / `vendor` / `device_type` が空でないことを検証する（空の場合は `ParseError::InvalidNiks`）。

## 関連ドキュメント

- [.nksf ファイルフォーマット概要](nksf-file-format.md)
- [NICA チャンク仕様](nica-chunk.md)
- [MessagePack エンコーディング](messagepack-encoding.md)
