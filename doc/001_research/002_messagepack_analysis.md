# MessagePackデータ構造解析

## 調査日時
2026-01-13

## 概要

.nksfファイルのNISIチャンクとNICAチャンクは、MessagePack形式でエンコードされている。このドキュメントでは、具体的なバイトパターンとデータ構造を解析する。

## MessagePack基礎

### 基本型のエンコーディング

| 型           | バイト       | 説明                                |
|--------------|-------------|-------------------------------------|
| fixstr       | 0xa0-0xbf   | 文字列（長さは下位5ビット）          |
| fixarray     | 0x90-0x9f   | 配列（要素数は下位4ビット）          |
| fixmap       | 0x80-0x8f   | マップ（要素数は下位4ビット）        |
| false        | 0xc2        | ブール値 false                       |
| true         | 0xc3        | ブール値 true                        |
| positive int | 0x00-0x7f   | 正の整数（値は直接）                |
| str8         | 0xd9 + len  | 8ビット長の文字列                   |

## NISIチャンク（メタデータ）の構造

### トップレベルマップ

```
0x8b (fixmap with 11 elements)
├── 0xad "__ni_internal" (13文字の文字列)
│   └── 0xa4 "BRIB" + nested data
├── 0xa6 "author" (6文字の文字列)
│   └── 0xb2 "Torsten Fassbender" (18文字の文字列)
├── 0xa9 "bankchain" (9文字の文字列)
│   └── 0x93 (fixarray with 3 elements)
│       ├── 0xa9 "Massive X"
│       ├── 0xb1 "Massive X Library"
│       └── 0xa0 "" (空文字列)
├── 0xaa "characters" (10文字の文字列)
│   └── 0x93 (fixarray with 3 elements)
│       ├── 0xa9 "Synthetic"
│       ├── 0xa4 "Dark"
│       └── 0xa9 "Distorted"
├── 0xa7 "comment" (7文字の文字列)
│   └── 0xb5 "Massive X Library 1.4"
├── 0xaa "deviceType" (10文字の文字列)
│   └── 0xa4 "INST"
├── 0xa5 "modes" (5文字の文字列)
│   └── 0x92 (fixarray with 2 elements)
├── 0xa4 "name" (4文字の文字列)
│   └── 0xa9 "Abandoned"
├── 0xa5 "types" (5文字の文字列)
│   └── 0x91 (fixarray with 1 element)
│       └── 0x92 (fixarray with 2 elements)
│           ├── 0xaa "Synth Misc"
│           └── 0xa2 "FX"
├── 0xa4 "uuid" (4文字の文字列)
│   └── 0xd9 0x24 (str8 with 36 bytes) + UUID文字列
└── 0xa6 "vendor" (6文字の文字列)
    └── 0xb2 "Native Instruments"
```

## NICAチャンク（パラメータ）の構造

### トップレベルマップ

```
0x81 (fixmap with 1 element)
└── 0xa3 "ni8" (3文字の文字列)
    └── 0x92 (fixarray with 2 elements)
        ├── [0] 0x98 (fixarray with 24 elements) - パラメータリスト
        └── [1] ... (追加データ)
```

### パラメータ要素の構造

各パラメータは以下の構造:

```
0x84 (fixmap with 4 elements)
├── 0xa8 "autoname" (8文字の文字列)
│   └── 0xc3 (true) または 0xc2 (false)
├── 0xa2 "id" (2文字の文字列)
│   └── 0x00-0x7f (正の整数)
├── 0xa4 "name" (4文字の文字列)
│   └── 0xa* ... (文字列 - パラメータ名)
└── 0xa5 "vflag" (5文字の文字列)
    └── 0xc3 (true) または 0xc2 (false)
```

### Abandoned.nksfのパラメータ例

```
Parameter 0:
  autoname: true (0xc3)
  id: 0
  name: "WT Pos 1"
  vflag: false (0xc2)

Parameter 1:
  autoname: true
  id: 1
  name: "WT Pos 2"
  vflag: false

Parameter 2:
  autoname: true
  id: 2
  name: "Osc 1 Lvl"
  vflag: false

Parameter 3:
  autoname: true
  id: 3
  name: "Osc 2 Lvl"
  vflag: false

Parameter 4:
  autoname: true
  id: 4
  name: "Filter"
  vflag: false

Parameter 5:
  autoname: true
  id: 5
  name: "Excite"
  vflag: false

Parameter 6:
  autoname: true
  id: 6
  name: "Delay"
  vflag: false
```

## バイト解析例

### NISIチャンク開始部分

```hex
Offset  Bytes                               Decoded
------  --------------------------------    ------------------
0x14    8b                                  fixmap (11 elements)
0x15    ad                                  fixstr (13 chars)
0x16    5f 5f 6e 69 5f 69 6e 74 65 72      "__ni_internal"
        6e 61 6c
0x22    a4                                  fixstr (4 chars)
0x23    42 52 49 42                         "BRIB"
0x27    a6                                  fixstr (6 chars)
0x28    61 75 74 68 6f 72                   "author"
0x2e    b2                                  fixstr (18 chars)
0x2f    54 6f 72 73 74 65 6e 20 46 61      "Torsten Fassbender"
        73 73 62 65 6e 64 65 72
```

### NICAチャンク開始部分（Abandoned.nksf、オフセット0x170）

```hex
Offset  Bytes                               Decoded
------  --------------------------------    ------------------
0x170   81                                  fixmap (1 element)
0x171   a3                                  fixstr (3 chars)
0x172   6e 69 38                            "ni8"
0x175   92                                  fixarray (2 elements)
0x176   98                                  fixarray (24 elements)
0x177   84                                  fixmap (4 elements) - Parameter 0
0x178   a8                                  fixstr (8 chars)
0x179   61 75 74 6f 6e 61 6d 65             "autoname"
0x181   c3                                  true
0x182   a2                                  fixstr (2 chars)
0x183   69 64                               "id"
0x185   00                                  0 (positive int)
0x186   a4                                  fixstr (4 chars)
0x187   6e 61 6d 65                         "name"
0x18b   a8                                  fixstr (8 chars)
0x18c   57 54 20 50 6f 73 20 31             "WT Pos 1"
0x194   a5                                  fixstr (5 chars)
0x195   76 66 6c 61 67                      "vflag"
0x19a   c2                                  false
```

## Rustでの実装方針

### 推奨アプローチ

1. **rmp-serde** を使用した構造体への自動デシリアライズ
   - Serde Deriveマクロで構造体を定義
   - `rmp_serde::from_slice()` でデシリアライズ

2. **カスタムデシリアライザー**（必要に応じて）
   - 特殊なフォーマットやバージョン互換性が必要な場合

### 構造体定義例

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct NisiMetadata {
    #[serde(rename = "__ni_internal")]
    ni_internal: NiInternal,
    author: String,
    bankchain: Vec<String>,
    characters: Vec<String>,
    comment: String,
    #[serde(rename = "deviceType")]
    device_type: String,
    modes: Vec<String>,
    name: String,
    types: Vec<Vec<String>>,
    uuid: String,
    vendor: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct NiInternal {
    #[serde(rename = "BRIB")]
    brib: serde_json::Value, // または適切な型
}

#[derive(Debug, Deserialize, Serialize)]
struct NicaData {
    ni8: Vec<serde_json::Value>, // または [Vec<Parameter>, AdditionalData]
}

#[derive(Debug, Deserialize, Serialize)]
struct Parameter {
    autoname: bool,
    id: u32,
    name: String,
    vflag: bool,
}
```

## 注意点

1. **エンディアンネス**: MessagePackは通常ビッグエンディアンだが、整数の格納方法に注意
2. **バージョン管理**: チャンクにバージョン情報があるため、将来的な変更に対応できる設計が必要
3. **未知のフィールド**: `__ni_internal` や追加データなど、完全に解析されていない部分がある
