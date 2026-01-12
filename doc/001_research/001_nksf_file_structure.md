# .nksfファイルフォーマット調査

## 調査日時
2026-01-13

## 対象ファイル
- `Abandoned.nksf` (32,736 bytes)
- `Alien Contact.nksf` (29,740 bytes)

## ファイルフォーマット概要

.nksfファイルは、**RIFF (Resource Interchange File Format)** をベースとしたバイナリフォーマットである。

## RIFFファイル構造

### 基本構造

```
Offset  | Size | Content                | Description
--------|------|------------------------|------------------
0x00    | 4    | "RIFF"                 | RIFFヘッダー識別子
0x04    | 4    | file_size - 8          | ファイルサイズ（リトルエンディアン）
0x08    | 4    | "NIKS"                 | フォーマット識別子
0x0C    | ...  | Chunks                 | チャンクデータ
```

### フォーマット識別子

- **"NIKS"**: Native Instruments Komplete Sound の略と推測

## チャンク構造

.nksfファイルには以下のチャンクが含まれる:

### 1. NISI チャンク (Metadata)

```
Offset  | Size | Content                | Description
--------|------|------------------------|------------------
0x0C    | 4    | "NISI"                 | チャンクID
0x10    | 4    | chunk_size             | チャンクサイズ（リトルエンディアン）
0x14    | 4    | version (0x00000001)   | バージョン番号
0x18    | ...  | MessagePack data       | メタデータ（MessagePack形式）
```

#### メタデータフィールド（MessagePack）

- `__ni_internal`: 内部データ
  - `BRIB`: 不明（おそらくバイナリリソース情報ブロック）
- `author`: 作者名（例: "Torsten Fassbender", "John Valasis"）
- `bankchain`: バンクチェーン（配列）
  - 例: ["Massive X", "Massive X Library", ""]
- `characters`: キャラクタータグ（配列）
  - 例: ["Synthetic", "Dark", "Distorted"]
- `comment`: コメント（例: "Massive X Library 1.4"）
- `deviceType`: デバイスタイプ（"INST"）
- `modes`: モード（配列）
  - 例: ["_Torsten Fassbender", "__Best of the Rest"]
- `name`: プリセット名（例: "Abandoned", "Alien Contact"）
- `types`: タイプ（配列）
  - 例: [["Synth Misc"], ["Synth Misc", "FX"]]
- `uuid`: UUID（例: "f890b345-58f9-4f54-815e-8709154700s6e"）
- `vendor`: ベンダー名（"Native Instruments"）

### 2. NICA チャンク (Parameters)

```
Offset  | Size | Content                | Description
--------|------|------------------------|------------------
?       | 4    | "NICA"                 | チャンクID
?       | 4    | chunk_size             | チャンクサイズ（リトルエンディアン）
?       | 4    | version (0x00000001)   | バージョン番号
?       | ...  | MessagePack data       | パラメータデータ（MessagePack形式）
```

#### パラメータデータ（MessagePack）

- `ni8`: 配列
  - 各要素はパラメータ情報を含む構造体:
    - `autoname`: bool（自動命名フラグ）
    - `id`: int（パラメータID）
    - `name`: string（パラメータ名）
    - `vflag`: bool（可視性フラグ）

例（Abandoned.nksf）:
- ID 0: "WT Pos 1"
- ID 1: "WT Pos 2"
- ID 2: "Osc 1 Lvl"
- ID 3: "Osc 2"

例（Alien Contact.nksf）:
- ID 0: "WT Pos"
- ID 1: "Width"
- ID 2: "Cutoff"
- ID 3: "Frq Shift"
- ID 4: "Crush"

## データフォーマット詳細

### MessagePack

NISIチャンクとNICAチャンクのデータは **MessagePack** フォーマットでシリアライズされている。

MessagePackは効率的なバイナリシリアライゼーションフォーマットで、以下のバイトパターンで識別できる:

- `0xa4` - `0xbf`: fixstr（固定長文字列、長さは下位バイトにエンコード）
- `0x90` - `0x9f`: fixarray（固定長配列）
- `0x80` - `0x8f`: fixmap（固定長マップ）
- `0xc2`: false
- `0xc3`: true
- `0xd9`: str8（8ビット長文字列）

## バイナリダンプ例

### Abandoned.nksf (先頭512バイト)

```
00000000  52 49 46 46 d8 7f 00 00  4e 49 4b 53 4e 49 53 49  |RIFF....NIKSNISI|
00000010  51 01 00 00 01 00 00 00  8b ad 5f 5f 6e 69 5f 69  |Q.........__ni_i|
00000020  6e 74 65 72 6e 61 6c a4  42 52 49 42 a6 61 75 74  |nternal.BRIB.aut|
00000030  68 6f 72 b2 54 6f 72 73  74 65 6e 20 46 61 73 73  |hor.Torsten Fass|
...
```

### Alien Contact.nksf (先頭512バイト)

```
00000000  52 49 46 46 24 74 00 00  4e 49 4b 53 4e 49 53 49  |RIFF$t..NIKSNISI|
00000010  30 01 00 00 01 00 00 00  8b ad 5f 5f 6e 69 5f 69  |0.........__ni_i|
00000020  6e 74 65 72 6e 61 6c a4  42 52 49 42 a6 61 75 74  |nternal.BRIB.aut|
00000030  68 6f 72 ac 4a 6f 68 6e  20 56 61 6c 61 73 69 73  |hor.John Valasis|
...
```

## Rustクレート推奨

### RIFFフォーマット解析

- **riff** (v2.0.0)
  - RIFFファイルの読み書き用ユーティリティ
  - チャンクベースのアクセス

### MessagePackデシリアライゼーション

- **rmp** (v0.8.15)
  - Pure Rust MessagePack実装
- **rmp-serde** (v1.3.1)
  - Serdeサポート（構造体への自動マッピング）

## 次のステップ

1. RIFFチャンクリーダーの実装
2. MessagePackデシリアライザーの実装
3. メタデータ構造体の定義
4. パラメータ構造体の定義
5. 統合パーサーの実装
