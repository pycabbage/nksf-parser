# PLIDチャンク完全解析

## 調査日時
2026-01-13

## ✓ 解析完了

**全73バイト（最大）の意味を完全に特定しました。**

## 概要

PLIDチャンク（Plugin ID）は、プラグイン識別情報を格納するMessagePackエンコードされたチャンク。

## チャンク構造

### 基本情報（Abandoned.nksf）

```
Offset  | Size | Content                | Description
--------|------|------------------------|------------------
0x3b2   | 4    | "PLID"                 | チャンクID
0x3b6   | 4    | 0x49 (73 bytes)        | チャンクサイズ（リトルエンディアン）
0x3ba   | 4    | 0x01 (1)               | バージョン（リトルエンディアン）
0x3be   | 69   | MessagePack data       | プラグイン情報
```

## MessagePackデータ構造

### トップレベルマップ

```
0x83 (fixmap with 3 elements)
├── "VST.magic" (0xa9 = fixstr 9)
│   └── 0xce 0x4e 0x69 0x24 0x48 (uint32 = 0x4824694e = 1210581326)
├── "pluginName" (0xaa = fixstr 10)
│   └── "Massive X" (0xa9 = fixstr 9)
└── "pluginVendor" (0xac = fixstr 12)
    └── "Native Instruments" (0xb2 = fixstr 18)
```

## バイトダンプ詳細

```hex
Offset  Bytes                               Decoded
------  --------------------------------    ------------------
0x3b2   50 4c 49 44                         "PLID"
0x3b6   49 00 00 00                         73 (chunk size, LE)
0x3ba   01 00 00 00                         1 (version, LE)

# MessagePack data starts
0x3be   83                                  fixmap (3 elements)
0x3bf   a9                                  fixstr (9 chars)
0x3c0   56 53 54 2e 6d 61 67 69 63          "VST.magic"
0x3c9   ce                                  uint32
0x3ca   4e 69 24 48                         0x4824694e (big-endian in msgpack)
0x3ce   aa                                  fixstr (10 chars)
0x3cf   70 6c 75 67 69 6e 4e 61 6d 65      "pluginName"
0x3d9   a9                                  fixstr (9 chars)
0x3da   4d 61 73 73 69 76 65 20 58          "Massive X"
0x3e3   ac                                  fixstr (12 chars)
0x3e4   70 6c 75 67 69 6e 56 65 6e 64      "pluginVendor"
        6f 72
0x3f0   b2                                  fixstr (18 chars)
0x3f1   4e 61 74 69 76 65 20 49 6e 73      "Native Instruments"
        74 72 75 6d 65 6e 74 73
```

## データフィールド

### VST.magic
- **型**: uint32（4バイト符号なし整数）
- **値**: 0x4824694e = 1,210,581,326
- **目的**: VST プラグインのマジックナンバー（識別子）
- **エンコーディング**: MessagePackのuint32（ビッグエンディアン）

### pluginName
- **型**: 文字列
- **値**: "Massive X"
- **目的**: プラグインの名称

### pluginVendor
- **型**: 文字列
- **値**: "Native Instruments"
- **目的**: プラグインのベンダー名

## Rust構造体定義

```rust
/// PLIDチャンク（Plugin ID）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlidData {
    /// VSTマジックナンバー
    #[serde(rename = "VST.magic")]
    pub vst_magic: u32,

    /// プラグイン名
    #[serde(rename = "pluginName")]
    pub plugin_name: String,

    /// プラグインベンダー名
    #[serde(rename = "pluginVendor")]
    pub plugin_vendor: String,
}
```

## 他のプリセットでの検証

### Alien Contact.nksf

同様の構造を確認すべき：
- VST.magic の値が同じか
- pluginName が "Massive X" か
- pluginVendor が "Native Instruments" か

## 実装方針

1. NISI/NICAチャンクと同じパターンでパーサーを実装
2. バージョン情報（4バイト）+ MessagePackデータ
3. `rmp_serde::from_slice()` でデシリアライズ
4. 全73バイトが正しく解析されることを確認

## 参考情報

- WebFetch結果: PLIDチャンクはMessagePackエンコードされている（確認済み）
- NISI/NICAと同じ解析パターンを適用可能

## プリセット間の違い

### 完全なPLID（73バイト）- Abandoned.nksf

```json
{
  "VST.magic": 1315513416,
  "pluginName": "Massive X",
  "pluginVendor": "Native Instruments"
}
```

### 簡易PLID（20バイト）- Alien Contact.nksf、All Rise.nksf

```json
{
  "VST.magic": 1315513416
}
```

## 結論

PLIDチャンクは2つのバリエーションが存在：
1. **完全版**（73バイト）：VST.magic + pluginName + pluginVendor
2. **簡易版**（20バイト）：VST.magicのみ

どちらも同じ構造（バージョン + MessagePack）で解析可能。

**✓ 全バイトの意味を特定完了**
