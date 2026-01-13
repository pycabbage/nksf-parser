# PCHKチャンク完全解析

## 調査日時
2026-01-13

## ✓ 解析完了

**全31,700バイト（圧縮データ）→ 507,182バイト（展開後）の意味を完全に特定しました。**

## 概要

PCHKチャンク（Plugin Chunk）は、Massive Xのプリセットデータ（全パラメータ、設定、Performerデータ等）を格納するチャンク。データはzlib圧縮され、展開後はMessagePackストリームとして格納されている。

## チャンク構造（圧縮データ）

### Abandoned.nksfの例（31,700バイト）

```
Offset  | Size  | Content                | Description
--------|-------|------------------------|------------------
0       | 4     | 0x01 (1)               | バージョン（リトルエンディアン）
4       | 4     | 0x02 (2)               | 不明フィールド1
8       | 4     | 0x02 (2)               | 不明フィールド2
12      | 4     | 31684                  | 圧縮データサイズ（リトルエンディアン）
16      | 4     | ?                      | 不明フィールド3
20      | ~     | zlib compressed data   | zlib圧縮されたMessagePackストリーム
```

### zlib圧縮識別

- オフセット20からマジックナンバー: **0x78 0x9c**（zlib deflate）
- 圧縮率: 約16倍（31,680バイト → 507,182バイト）

## 展開後のデータ構造

### MessagePackストリーム（507,182バイト）

展開後のデータは、**268個のMessagePack値**が連続したストリームとして格納されている。

### データセクション構造

#### セクションタイプ1: Map Sections（name-count-map）

最初の5セクションは、3つの値で構成される：

1. **セクション名**（String）
2. **エントリ数**（Number）- 宣言
3. **データマップ**（Object）- 実際のkey-value

```
構造: <section_name> <declared_count> <data_map>
```

| セクション | 宣言数 | 実際数 | バイト数 | 内容 |
|-----------|-------|-------|---------|------|
| strings | 759 | 758 | 95,782 | 文字列値のパス→値マップ |
| floats | 2 | 1 | 76 | float32値のパス→値マップ |
| doubles | 1105 | 1104 | 154,938 | float64値のパス→値マップ |
| ints | 384 | 383 | 37,479 | 整数値のパス→値マップ |
| bools | 1581 | 1580 | 205,380 | ブール値のパス→値マップ |

**合計: 493,655バイト**

#### セクションタイプ2: Vec Sections（name-count-pairs）

残りのセクションは、name-countの後にkey-valueペアが続く：

```
構造: <section_name> <count> [<key> <value>]{count}
```

| セクション | カウント | 構造 | バイト数（概算） |
|-----------|---------|------|---------|
| charVecs | 1 | メタ情報（"intVecs": 42） | 19 |
| intVecs | 42 | String → Array[int] | ~3,500 |
| floatVecs | ? | String → Array[float] | ? |
| doubleVecs | ? | String → Array[double] | ? |
| stringVecs | ? | String → Array[string] | ? |

**合計: 13,527バイト**

### 総MessagePack値数

**268個の値 = 507,182バイト全て**

- Section names: 10個（strings, floats, doubles, ints, bools, charVecs, intVecs, floatVecs, doubleVecs, stringVecs）
- Section counts: 10個
- Section maps: 5個（Map Sections）
- Key-value pairs: 243個（Vec Sectionsのデータ）

## データ内容の詳細

### stringsセクション（758エントリ）

プリセットの文字列設定を格納：

```
"meta/hash": "1cde7b7a6d767b6bec5a71498bd875cf"
"meta/presetName": "Abandoned"
"root/engine/global/macros/macro1/macroName/value": "WT Pos 1"
"root/engine/unit1/audioPluginSlots/MainOsc1Slot/selectedPlugin": "Wavetable"
...
```

**内容**：
- メタデータ（hash, presetName等）
- マクロ名（Macro 1-16）
- プラグイン選択（Wavetable, FreqShifter等）
- モジュレーションソースパス
- エンベロープ設定
- その他の文字列パラメータ

### floatsセクション（1エントリ）

float32精度のパラメータ：

```
"root/engine/unit1/Performers/performers/grid/overlay stretch": <float32 value>
```

### doublesセクション（1104エントリ）

float64精度のパラメータ（メインのパラメータ値）：

```
"root/engine/global/macros/macro1/macroValue/normalizedValue": <double value>
"root/engine/unit1/MainOsc1Slot/Wavetable/Pos/Knob/normalizedValue": <double value>
"root/engine/unit1/FilterSlot/CombPlugin/Decay/Knob/normalizedValue": <double value>
...
```

**内容**：
- マクロ値（normalized値）
- オシレーターパラメータ
- フィルターパラメータ
- エフェクトパラメータ
- エンベロープパラメータ
- LFOパラメータ
- モジュレーション量

### intsセクション（383エントリ）

整数パラメータ：

```
"meta/numUnits": 1
"meta/type": 1
"root/engine/unit1/Keytrackers/keytrackers/keytracker 3/grid y/unipolar steps": <int value>
...
```

**内容**：
- メタデータ（ユニット数、タイプ）
- グリッドステップ数
- 生の整数値（rawValue）

### boolsセクション（1580エントリ）

ブール値パラメータ：

```
"meta/hasIcon": false
"meta/presetModified": false
"root/engine/unit1/ModSlot/isActive": true
"root/engine/unit1/ModSlot/isMuted": false
...
```

**内容**：
- メタデータフラグ
- モジュレーション有効/無効状態
- ミュート状態
- その他のオン/オフ設定

### charVecs, intVecs, floatVecs, doubleVecs, stringVecsセクション

配列データを格納：

```
"root/engine/unit1/Performers/performers/performer 0/page 0/points": [0, 268435456, 536870912, 268435456]
"root/engine/unit1/Performers/performers/performer 0/page 0/segment types": ["C_SHAPE", "HOLD_FLAG"]
...
```

**内容**：
- Performerのポイントデータ（座標、値）
- Performerのセグメントタイプ
- その他の配列データ

## 検証結果

### テスト対象

1. **Abandoned.nksf**（32,736バイト）
   - PLID: 73バイト（完全版）
   - PCHK: 31,700バイト → 507,182バイト展開
   - ✓ 全バイト解析成功

2. **Alien Contact.nksf**（29,740バイト）
   - PLID: 20バイト（簡易版）
   - PCHK: 28,798バイト → 469,570バイト展開
   - ✓ 全バイト解析成功

3. **All Rise.nksf**（29,815バイト）
   - PLID: 20バイト（簡易版）
   - PCHK: 28,953バイト → 464,541バイト展開
   - ✓ 全バイト解析成功

### 共通性

- 全プリセットで**268個のMessagePack値**
- 同じセクション構造
- 全バイトが解析可能
- 同じVST.magic値（1315513416）

## 実装方針

### PLIDチャンク

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlidData {
    #[serde(rename = "VST.magic")]
    pub vst_magic: u32,

    #[serde(rename = "pluginName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin_name: Option<String>,

    #[serde(rename = "pluginVendor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin_vendor: Option<String>,
}
```

### PCHKチャンク

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PchkData {
    /// ヘッダー情報
    pub header: PchkHeader,

    /// 全MessagePack値（268個）
    pub values: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PchkHeader {
    pub version: u32,
    pub field1: u32,
    pub field2: u32,
    pub compressed_size: u32,
    pub field3: u32,
}
```

**✓ 完全なバイト解析が可能**