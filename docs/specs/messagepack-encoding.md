# .nksf における MessagePack エンコーディング

## 概要

.nksf ファイルの NISI / NICA / PLID 各チャンク、および PCHK チャンクの zlib 展開後データは **MessagePack** 形式でエンコードされている。本ドキュメントでは .nksf で実際に使われている型のエンコーディングと、実バイト列のデコード例を示す。

> 実装: デシリアライズは [`lib/src/msgpack_utils.rs`](../../lib/src/msgpack_utils.rs)（`parse_versioned_msgpack`）と [`lib/src/pchk_parser.rs`](../../lib/src/pchk_parser.rs) で行い、クレートには [`rmp-serde`](https://crates.io/crates/rmp-serde) を使用している。

## 使用される型のエンコーディング

.nksf で観測される MessagePack 型の一覧:

| 型 | 先頭バイト | 説明 |
|---|---|---|
| positive fixint | `0x00`–`0x7f` | 0〜127 の正の整数。値がそのまま先頭バイト |
| fixmap | `0x80`–`0x8f` | 要素数 15 以下のマップ。要素数 = 下位 4 ビット |
| fixarray | `0x90`–`0x9f` | 要素数 15 以下の配列。要素数 = 下位 4 ビット |
| fixstr | `0xa0`–`0xbf` | 長さ 31 バイト以下の文字列。バイト長 = 下位 5 ビット |
| false | `0xc2` | ブール値 false |
| true | `0xc3` | ブール値 true |
| str8 | `0xd9` + len(1B) | 長さ 255 バイト以下の文字列。例: UUID（36 バイト） |
| uint32 | `0xce` + 値(4B, BE) | 符号なし 32bit 整数。例: PLID の `VST.magic` |

備考:

- 文字列は UTF-8。fixstr のプレフィックスは `0xa0 + バイト長`（例: 13 文字なら `0xad`、18 文字なら `0xb2`）。
- マップ/配列の「要素数」はキーと値のペア数（マップ）または値の個数（配列）。マップの場合 `0x8b` は 11 ペアを意味する。
- 浮動小数点（PCHK の floats/doubles セクション内の値）も通常の MessagePack float エンコーディングに従う。

## エンディアンの注意

同一ファイル内でエンディアンの慣習が混在している点に注意すること。

| 項目 | エンディアン | 例 |
|---|---|---|
| RIFF ヘッダのサイズ、チャンクサイズ、チャンク version、PCHK 固有ヘッダ | **リトルエンディアン** (u32 LE) | `"NISI"` 直後の `51 01 00 00` = 337 |
| MessagePack 内の整数（positive fixint を除く多バイト整数） | **ビッグエンディアン** | `VST.magic`: `ce 4e 69 24 48` = `0x4E692448` |

つまりチャンクヘッダを読むときは `u32::from_le_bytes`、MessagePack デコーダに渡すときはそのままバイト列を与えればよい（デコーダが BE を解釈する）。実装でもこの区別に従っている（[riff_reader.rs](../../lib/src/riff_reader.rs)、[msgpack_utils.rs](../../lib/src/msgpack_utils.rs)）。

## デコード例 1: NISI チャンク先頭

`Abandoned.nksf` のオフセット `0x18`（NISI チャンク MessagePack データ部の先頭。チャンクは `0x0C` から始まり、ID 4 B + サイズ 4 B + バージョン 4 B の後ろにペイロードが続く）から:

```hex
Offset  Bytes                               Decoded
------  --------------------------------    ------------------------------
0x18    8b                                  fixmap (要素数 11)
0x19    ad                                  fixstr (13文字)
0x1a    5f 5f 6e 69 5f 69 6e 74 65 72       "__ni_internal"
        6e 61 6c
0x26    a4                                  fixstr (4文字)
0x27    42 52 49 42                         "BRIB"
        ...                                 (__ni_internal の値: 内部データ)
...     a6                                  fixstr (6文字)
        61 75 74 68 6f 72                   "author"
        b2                                  fixstr (18文字)
        54 6f 72 73 74 65 6e 20 46 61       "Torsten Fassbender"
        73 73 62 65 6e 64 65 72
```

読み進めると、`bankchain` は `93`（fixarray 要素数 3）、`types` は `91`（fixarray 要素数 1）の中に `92`（fixarray 要素数 2: "Synth Misc", "FX"）、`uuid` は `d9 24`（str8 36 バイト）というようにエンコードされている。全体構造は [nisi-chunk.md](nisi-chunk.md) を参照。

## デコード例 2: NICA パラメータエントリ

`Abandoned.nksf` のオフセット `0x171`（NICA チャンク MessagePack 部の先頭）から、最初のパラメータ 1 個分:

```hex
Offset  Bytes                               Decoded
------  --------------------------------    ------------------------------
0x171   81                                  fixmap (要素数 1)
0x172   a3                                  fixstr (3文字)
0x173   6e 69 38                            "ni8"
0x176   92                                  fixarray (要素数 2)
0x177   98                                  fixarray (要素数 8)
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

この 1 エントリは次の JSON 相当のデータを表す:

```json
{ "autoname": true, "id": 0, "name": "WT Pos 1", "vflag": false }
```

`id` の `0x00` は positive fixint（値 0 そのもの）であることに注意。0〜127 の整数はプレフィックスなしの 1 バイトで表現されるため、パラメータ ID 番号のような小さい値は非常にコンパクトに格納される。全体構造は [nica-chunk.md](nica-chunk.md) を参照。

## rmp-serde を採用した理由

- **serde との統合**: `#[derive(Serialize, Deserialize)]` した構造体（`NisiMetadata` / `PlidData` 等）へ `rmp_serde::from_slice()` 一発でデシリアライズでき、フィールド名の対応は `#[serde(rename = "...")]` で宣言的に書ける（例: `deviceType` → Rust の `device_type`）。
- **省略可能フィールドの扱い**: PLID の `pluginName` / `pluginVendor` のような optional なキーは `Option<T>` + `#[serde(default)]` で自然に扱える。
- **純 Rust 実装**: 追加のネイティブ依存が不要。
- **部分ストリーム読み取り**: PCHK のような「1 つのバッファに複数の MessagePack 値が連結したストリーム」も、`rmp_serde::from_read(&mut cursor)` を繰り返し呼ぶことで先頭から順に読み進められる（[pchk-chunk.md](pchk-chunk.md) 参照）。

なお、ライブラリ本体 (`nksf-parser`) は JSON 出力を行わない設計であり、内部での柔軟な保持のために一部の生データ（PCHK values、`NiInternal`）に `serde_json::Value` を用いているが、これは出力形式としての JSON とは無関係である。

## 関連ドキュメント

- [.nksf ファイルフォーマット概要](nksf-file-format.md)
- [NISI チャンク仕様](nisi-chunk.md)
- [NICA チャンク仕様](nica-chunk.md)
- [PLID チャンク仕様](plid-chunk.md)
- [PCHK チャンク仕様](pchk-chunk.md)
