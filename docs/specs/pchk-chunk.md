# PCHK チャンク（プラグイン状態本体）

## 概要

PCHK チャンクは Massive X プリセットの本体データ、すなわちシンセの全パラメータ値・Performer データ等の完全なプラグイン状態を格納する。ペイロードは「固有ヘッダ 20 バイト + zlib 圧縮された MessagePack ストリーム」という構成で、展開後は数百 KB の連続 MessagePack 値列となる。圧縮率は約 16 倍（観測値）。

> 実装: [`lib/src/pchk_parser.rs`](../../lib/src/pchk_parser.rs)（`parse_pchk_chunk`）、構造体は [`lib/src/types.rs`](../../lib/src/types.rs) の `PchkData` / `PchkHeader`

## ヘッダ（20 バイト）

チャンク共通ヘッダ（[nksf-file-format.md](nksf-file-format.md)）の後ろに、PCHK 固有の 20 バイトヘッダが続く。すべてリトルエンディアン u32 である。

| Offset | Size | Content | Description |
|--------|------|---------|-------------|
| +0x00 | 4 | version | バージョン。観測値: 常に 1 |
| +0x04 | 4 | field1 | 用途未特定。観測値: 全プリセットで 2 |
| +0x08 | 4 | field2 | 用途未特定。観測値: 0〜3 の可変値 |
| +0x0C | 4 | compressed_size | zlib 圧縮データのバイト数と推定される（実装では読み飛ばされ、チャンク末尾までを展開する）。観測例: 31684 |
| +0x10 | 4 | field3 | 用途未特定、可変値 |

- `field1` / `field2` / `field3` の意味は特定できていない（観測値のみ記載）。実装では構造体メンバとして生値を保持する。
- `version != 1` の場合は `ParseError::InvalidNiks` を返す。

## zlib 圧縮データ

- ヘッダ直後（オフセット 20）からチャンク末尾までが zlib ストリームで、先頭 2 バイトは zlib のマジック **`78 9c`**（deflate / デフォルト圧縮レベル）で始まる。
- 展開後サイズは約 507 KB（`Abandoned.nksf` では圧縮データ 約31,680 B → 展開後 507,182 B、圧縮率 約16倍）。
- セキュリティ対策として、パーサーは展開サイズを 50 MB に制限している（Zip Bomb 対策。上限到達時は `ParseError::InvalidNiks`）。

> 実装: `flate2::read::ZlibDecoder` を使用

## 展開後: MessagePack ストリーム

展開後のデータは **268 個の MessagePack 値**が連続したストリームである（`Abandoned.nksf` / `Alien Contact.nksf` / `All Rise.nksf` のいずれでも 268 個で一致、観測値）。パーサーはストリーム末尾まで値を読み切り、残りバイトがあれば `ParseError::IncompleteParse` を返す。

### Map Sections パターン（name → count → data_map）

最初の 15 個の値は、**5 セクション × 3 値**の繰り返しからなる。

```
<section_name(String)> <declared_count(Number)> <data_map(Object)>
```

| # (値番号) | セクション名 | 宣言数 | 実際数 | 観測バイト数 |
|---|---|---|---|---|
| 0–2 | `strings` | 759 | 758 | 95,782 |
| 3–5 | `floats` | 2 | 1 | 76 |
| 6–8 | `doubles` | 1105 | 1104 | 154,938 |
| 9–11 | `ints` | 384 | 383 | 37,479 |
| 12–14 | `bools` | 1581 | 1580 | 205,380 |

- 各 `data_map` は「パス風キー → 型付きパラメータ値」のマップ。
- **宣言数は常に実際数より 1 大きい**（5 セクションすべてで一致する挙動。仕様上の意図は不明）。
- 5 セクション合計は 493,655 バイト（観測値）で、ストリーム全体の大部分を占める。

### Vec Sections パターン（name → count → key/value ペア列）

Map Sections の後に続く部分は配列値セクションである。

```
<section_name(String)> <count(Number)> [<key(String)> <value(Array)>]{count}
```

| セクション | 観測される構造 | 内容 |
|---|---|---|
| `charVecs` | count=1、値としてマップ `{"intVecs": 42}` | メタデータ。**次のセクション (`intVecs`) のエントリ数を宣言する** |
| `intVecs` | 宣言 42 / 実際 41 ペア（String → Array[int]） | Performer/Keytracker のポイント座標等（固定小数点整数列） |
| `floatVecs` | 宣言 41 / 実際 40 ペア（String → Array[float]） | Performer のセグメント bending factors 等 |
| `doubleVecs` | count=1 | 観測例ではデータペアを持たない（次セクション名への橋渡しのみ） |
| `stringVecs` | 宣言 40 / 実際 39 ペア（String → Array[string]） | Performer のセグメントタイプ、ページ名、タグ等 |

Vec 部分合計は約 13,527 バイト（観測値）。Vec Sections でも Map Sections と同様に「宣言数は実際のペア数より 1 大きい」という挙動が観測される（例: `intVecs` 宣言 42 → 実際 41 ペア）。

### 値数の内訳（268 個 = 全体）

- セクション名: 10 個（`strings`, `floats`, `doubles`, `ints`, `bools`, `charVecs`, `intVecs`, `floatVecs`, `doubleVecs`, `stringVecs`）
- カウント値: 10 個
- Map Section の data_map: 5 個
- Vec Section の key/value・メタデータペア: 243 個

## 各セクションの内容

キーはスラッシュ区切りのパス形式（例: `root/engine/unit1/...`）で、シンセ内部のパラメータツリーを表す。

### strings（観測 758 エントリ）
文字列パラメータ。
- メタデータ: `meta/hash`、`meta/presetName`
- マクロ名: `root/engine/global/macros/macro1/macroName/value`（"WT Pos 1" 等）
- マクロの割当先: `root/engine/global/macros/macro1/macroAssignment/oneToOneMappingPath`
- プラグイン選択: `root/engine/unit1/audioPluginSlots/MainOsc1Slot/selectedPlugin`（"Wavetable" 等）
- モジュレーションソースパス、エンベロープ設定 等

### floats（観測 1 エントリ）
float32 精度の単一パラメータ。
- `root/engine/unit1/Performers/performers/grid/overlay stretch`

### doubles（観測 1104 エントリ）
float64 精度のパラメータ。**シンの主要パラメータ値の大半がここ**に格納される。
- マクロ値: `root/engine/global/macros/macro1/macroValue/normalizedValue`
- オシレーター: `root/engine/unit1/audioPluginSlots/MainOsc1Slot/audioPlugins/Wavetable/Pos/Knob/parameterState/normalizedValue`
- フィルター: `.../FilterSlot/audioPlugins/CombPlugin/Decay/Knob/parameterState/normalizedValue` 等
- エフェクト、エンベロープ、LFO、モジュレーション量（`modulationAmountState/normalizedValue`）等

### ints（観測 383 エントリ）
整数パラメータ。
- メタデータ: `meta/numUnits`、`meta/type`、`meta/version`
- UI状態: `root/engine/frame/activeUnit/value`、`root/engine/frame/framePage/value`
- マクロ割当モード: `.../macroAssignment/assignmentMode`
- グリッドステップ数: `root/engine/unit1/Keytrackers/keytrackers/keytracker 0/grid y/unipolar steps`
- 生の整数値: `modulationAmountState/rawValue` 等

### bools（観測 1580 エントリ）
ブールフラグ。
- メタデータ: `meta/hasIcon`、`meta/presetModified`
- 有効/無効: `meta/unit0/isActive`、`.../modulationNode/isActive`、`.../modulationTargetNodeState/isActive`
- ミュート: `.../isMuted`、接続状態: `.../isConnected`
- グリッド表示: `root/engine/unit1/Performers/performers/grid/overlay enabled`

### Vec セクション群
配列データ。主に **Performer**（3 台 × 12 ページ）と Keytracker のエディタデータ。
- ポイント座標（intVecs）: `root/engine/unit1/Performers/performers/performer 0/page 0/points` → `[0, 268435456, 536870912, 268435456]`（32bit 固定小数点相当の整数列、観測例）
- Keytracker ポイント（intVecs）: `root/engine/unit1/Keytrackers/keytrackers/keytracker 1/points`
- セグメント bending factors（floatVecs）: `.../page 11/segment bending factors`
- セグメントタイプ（stringVecs）: `.../page 9/segment types` → `["C_SHAPE", "C_SHAPE|HOLD_FLAG"]` のような列挙値（`C_SHAPE` / `HOLD_FLAG` 等のフラグを `|` で組み合わせた文字列）
- ページ名（stringVecs）: `root/engine/unit1/genericAdapters/RemoteOctave/remote octave/page names`
- タグ（stringVecs）: `meta/tags`

## プリセット間の検証結果

複数プリセットで同一の構造が確認されている（観測値）。

| プリセット | PLID サイズ | PCHK 圧縮後 | 展開後 | MessagePack 値数 |
|---|---|---|---|---|
| Abandoned.nksf | 73（完全版） | 31,700 B | 507,182 B | 268 |
| Alien Contact.nksf | 20（最小版） | 28,798 B | 469,570 B | 268 |
| All Rise.nksf | 20（最小版） | 28,953 B | 464,541 B | 268 |

共通事項:

- いずれも **268 個の MessagePack 値**、同じセクション構造（5 Map Sections + 5 Vec Sections）、同じ順序
- 同じ `VST.magic`（1315513416。ただし一部プリセットでは 0 も観測される）
- PCHK ヘッダの `version`=1 / `field1`=2 は全プリセット共通、`field2` のみ 0〜3 で変動

## Rust 構造体

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PchkData {
    /// ヘッダー情報
    pub header: PchkHeader,

    /// 全MessagePack値
    /// 構造: [name1, count1, data1, name2, count2, data2, ...]
    pub values: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PchkHeader {
    /// バージョン（通常1）
    pub version: u32,
    /// 不明フィールド1（用途未特定、観測値は2）
    pub field1: u32,
    /// 不明フィールド2（用途未特定、観測値は0〜3で可変）
    pub field2: u32,
    /// 圧縮データサイズ（zlib圧縮後のバイト数）
    pub compressed_size: u32,
    /// 不明フィールド3（用途未特定、値は可変）
    pub field3: u32,
}
```

`values` は 268 個の値をセクション構造のまま `serde_json::Value` 列として保持する（セクション解釈は利用者側で行う設計）。パース処理には無限ループ防止のため値数上限（100,000）も設けられている。

## 関連ドキュメント

- [.nksf ファイルフォーマット概要](nksf-file-format.md)
- [PLID チャンク仕様](plid-chunk.md)
- [MessagePack エンコーディング](messagepack-encoding.md)
