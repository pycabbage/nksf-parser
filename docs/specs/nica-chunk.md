# NICA チャンク（パラメータアサインメント）

## 概要

NICA チャンクは、プラグインがホスト (DAW) に公開するパラメータのアサインメントを格納する。各パラメータの ID・表示名・フラグの一覧であり、DAW のオートメーションやクイックコントロールで参照される。ペイロードは MessagePack 形式で、エンコーディングの基礎は [messagepack-encoding.md](messagepack-encoding.md) を参照。

> 実装: [`lib/src/nica_parser.rs`](../../lib/src/nica_parser.rs)（`parse_nica_chunk`）、構造体は [`lib/src/types.rs`](../../lib/src/types.rs) の `NicaData` / `Parameter`

## チャンクレイアウト

`Abandoned.nksf` での観測値（チャンク全体で 589 バイト）:

| Offset（チャンク先頭基準） | Size | Content | Description |
|--------|------|---------|-------------|
| +0x00 | 4 | `"NICA"` | チャンク ID（ASCII）。ファイル上ではオフセット `0x165` に位置 |
| +0x04 | 4 | chunk_size | チャンクサイズ（リトルエンディアン u32）。観測例: 581 |
| +0x08 | 4 | version | チャンクバージョン（リトルエンディアン u32）。観測値: 1 |
| +0x0C | 577 | MessagePack データ | パラメータデータ |

MessagePack 部分の先頭バイトは `0x81`（要素数 1 の fixmap）。

## MessagePack 構造

トップレベル構造は次のとおり。

```
fixmap(1)
└── "ni8": fixarray(2)
    ├── [0] fixarray(N)   … パラメータリスト（Nはプリセットにより可変。観測例では8要素×2配列）
    └── [1] ...           … 追加データ
```

- マップの唯一のキーは `"ni8"`。
- 値は **2 要素の配列** で、第 1 要素がパラメータリスト、第 2 要素が追加データである。
- パラメータリストの長さ（= 公開パラメータ数）はプリセットごとに異なる。`Abandoned.nksf` の観測例では、ID 0〜7（"WT Pos 1"〜"Reverb"）と ID 8〜15（"Macro 9"〜"Macro 16"）の **2 つのサブ配列**（各 8 要素）に分割されて格納されており、合計 16 パラメータとなる。

### パラメータ要素の構造

リストの各要素は **fixmap(4)** で、次のキーを持つ。

| キー | 型 | 説明 |
|---|---|---|
| `autoname` | bool | 自動命名フラグ |
| `id` | 正の整数 (positive fixint) | パラメータ ID |
| `name` | 文字列 | パラメータ名 |
| `vflag` | bool | 可視性フラグ |

観測例（`Abandoned.nksf` の先頭パラメータ）:

- `{ autoname: true, id: 0, name: "WT Pos 1", vflag: false }`
- `{ autoname: true, id: 2, name: "Osc 1 Lvl", vflag: false }`
- `{ autoname: true, id: 7, name: "Reverb", vflag: false }`
- `{ autoname: true, id: 15, name: "Macro 16", vflag: false }`

別プリセット `Alien Contact.nksf` では `"WT Pos"` / `"Width"` / `"Cutoff"` / `"Frq Shift"` / `"Crush"` のような異なる名前セット・個数となり、構造は同じでも内容はプリセット依存である。

## デコード例

`Abandoned.nksf` のファイルオフセット `0x171` 以降（NICA チャンク MessagePack 部の先頭。NISI チャンクが `0x0C` + 345 B であるため、NICA の version フィールドは `0x16D`、ペイロードは `0x171` から始まる）:

```hex
Offset  Bytes                               Decoded
------  --------------------------------    ------------------------------
0x171   81                                  fixmap (要素数 1)
0x172   a3                                  fixstr (3文字)
0x173   6e 69 38                            "ni8"
0x176   92                                  fixarray (要素数 2)
0x177   98                                  fixarray (要素数 8) — パラメータ配列 [0]
0x178   84                                  fixmap (要素数 4) — Parameter 0
0x179   a8                                  fixstr (8文字)
0x17a   61 75 74 6f 6e 61 6d 65             "autoname"
0x182   c3                                  true
0x183   a2                                  fixstr (2文字)
0x184   69 64                               "id"
0x186   00                                  0 (positive fixint)
0x187   a4                                  fixstr (4文字)
0x188   6e 61 6d 65                         "name"
0x18c   a8                                  fixstr (8文字)
0x18d   57 54 20 50 6f 73 20 31             "WT Pos 1"
0x195   a5                                  fixstr (5文字)
0x196   76 66 6c 61 67                      "vflag"
0x19b   c2                                  false
```

## Rust 構造体

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NicaData {
    /// パラメータリストと追加データ
    #[serde(default)]
    pub ni8: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub autoname: bool,
    pub id: u32,
    pub name: String,
    pub vflag: bool,
}
```

`NicaData.ni8` は「パラメータリスト + 追加データ」という 2 要素構造をそのまま保持できるよう `Vec<serde_json::Value>` 型としている。パーサーはデシリアライズ後に各サブ配列内で `id` の重複を検証し、重複があれば `ParseError::InvalidNiks` を返す。

## 関連ドキュメント

- [.nksf ファイルフォーマット概要](nksf-file-format.md)
- [NISI チャンク仕様](nisi-chunk.md)
- [MessagePack エンコーディング](messagepack-encoding.md)
