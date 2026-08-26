# PLID チャンク（プラグイン ID）

## 概要

PLID チャンクは、プリセットの生成元プラグインを識別する情報を格納する。VST マジックナンバーを必須キーとし、プラグイン名・ベンダー名は省略され得る。ペイロードは MessagePack 形式で、エンコーディングの基礎は [messagepack-encoding.md](messagepack-encoding.md) を参照。

> 実装: [`lib/src/plid_parser.rs`](../../lib/src/plid_parser.rs)（`parse_plid_chunk`）、構造体は [`lib/src/types.rs`](../../lib/src/types.rs) の `PlidData`

## チャンクレイアウト

2 種類のバリアントが観測されている。

### 完全版（73 バイト、`Abandoned.nksf`）

| Offset（チャンク先頭基準） | Size | Content | Description |
|--------|------|---------|-------------|
| +0x00 | 4 | `"PLID"` | チャンク ID（ASCII）。ファイル上ではオフセット 0x3b2 に位置（観測値） |
| +0x04 | 4 | chunk_size | チャンクサイズ（リトルエンディアン u32）。観測値: `49 00 00 00` = 73 |
| +0x08 | 4 | version | チャンクバージョン（リトルエンディアン u32）。観測値: 1 |
| +0x0C | 69 | MessagePack データ | プラグイン識別マップ |

### 最小版（20 バイト、`Alien Contact.nksf` / `All Rise.nksf` 等）

| Offset（チャンク先頭基準） | Size | Content | Description |
|--------|------|---------|-------------|
| +0x00 | 4 | `"PLID"` | チャンク ID（ASCII） |
| +0x04 | 4 | chunk_size | 観測値: 20 |
| +0x08 | 4 | version | 観測値: 1 |
| +0x0C | 16 | MessagePack データ | `VST.magic` のみを含むマップ |

## MessagePack 構造

トップレベルは **fixmap** で、次のキーを持ち得る。

| キー | 型 | 必須 | 説明 |
|---|---|---|---|
| `"VST.magic"` | uint32 (`0xce`) | ○ | VST プラグインのマジックナンバー（識別子）。MessagePack 整数のため**ビッグエンディアン**で格納される |
| `"pluginName"` | 文字列 | − | プラグイン名。観測値は `"Massive X"` |
| `"pluginVendor"` | 文字列 | − | ベンダー名。観測値は `"Native Instruments"` |

### バリアント

| バリアント | チャンクサイズ | キー構成 |
|---|---|---|
| 完全版 | 73 バイト | `VST.magic` + `pluginName` + `pluginVendor`（fixmap 要素数 3） |
| 最小版 | 20 バイト | `VST.magic` のみ（fixmap 要素数 1） |

どちらも「version (u32 LE) + MessagePack」という共通構造であり、同一のパーサー経路で処理できる。

## デコード例

`Abandoned.nksf` の PLID チャンク（完全版）:

```hex
Offset  Bytes                               Decoded
------  --------------------------------    ------------------------------
0x3b2   50 4c 49 44                         "PLID"
0x3b6   49 00 00 00                         73 (chunk size, LE)
0x3ba   01 00 00 00                         1 (version, LE)

# MessagePack データ部（0x3be〜）
0x3be   83                                  fixmap (要素数 3)
0x3bf   a9                                  fixstr (9文字)
0x3c0   56 53 54 2e 6d 61 67 69 63          "VST.magic"
0x3c9   ce                                  uint32
0x3ca   4e 69 24 48                         0x4E692448 (= 1315513416, BE)
0x3ce   aa                                  fixstr (10文字)
0x3cf   70 6c 75 67 69 6e 4e 61 6d 65       "pluginName"
0x3d9   a9                                  fixstr (9文字)
0x3da   4d 61 73 73 69 76 65 20 58          "Massive X"
0x3e3   ac                                  fixstr (12文字)
0x3e4   70 6c 75 67 69 6e 56 65 6e 64 6f 72 "pluginVendor"
0x3f0   b2                                  fixstr (18文字)
0x3f1   4e 61 74 69 76 65 20 49 6e 73       "Native Instruments"
        74 72 75 6d 65 6e 74 73
```

### VST.magic の値について

- 観測値: **1315513416 = 0x4E692448**（MessagePack の uint32 としてビッグエンディアンのバイト列 `4E 69 24 48` で格納される。ASCII に読むと "Ni$H"）
- この値は Massive X プラグインの識別子であり、複数プリセット間で共通している（観測値）。
- 一部のプリセットでは `VST.magic` が `0` であるケースも観測されている。
- 注意: RIFF 側のサイズフィールドとはエンディアンが逆（[messagepack-encoding.md](messagepack-encoding.md) 参照）。

## Rust 構造体

実装では `pluginName` / `pluginVendor` を `Option<String>` として最小版にも対応している。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlidData {
    /// VSTマジックナンバー
    #[serde(rename = "VST.magic")]
    pub vst_magic: u32,

    /// プラグイン名（オプション）
    #[serde(rename = "pluginName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin_name: Option<String>,

    /// プラグインベンダー名（オプション）
    #[serde(rename = "pluginVendor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin_vendor: Option<String>,
}
```

## 関連ドキュメント

- [.nksf ファイルフォーマット概要](nksf-file-format.md)
- [PCHK チャンク仕様](pchk-chunk.md)
- [MessagePack エンコーディング](messagepack-encoding.md)
