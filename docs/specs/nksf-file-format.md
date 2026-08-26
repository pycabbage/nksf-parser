# .nksf ファイルフォーマット

## 概要

.nksf は Native Instruments **Massive X** のプリセットファイルである。全体は **RIFF (Resource Interchange File Format)** をベースとしたコンテナ構造をしており、内部に 4 種類のチャンク（`NISI` / `NICA` / `PLID` / `PCHK`）を格納する。メタデータ系チャンクは MessagePack でエンコードされており、詳細は [MessagePack エンコーディング](messagepack-encoding.md) を参照。

本ドキュメントはフォーマット全体の概要を示す。個々のチャンクの詳細は以下の各ドキュメントを参照すること。

| チャンク | 内容 | 詳細仕様 |
|---|---|---|
| `NISI` | プリセットメタデータ（名前・作者・タグ等） | [nisi-chunk.md](nisi-chunk.md) |
| `NICA` | ホスト向けパラメータアサインメント | [nica-chunk.md](nica-chunk.md) |
| `PLID` | プラグイン識別情報（VST.magic 等） | [plid-chunk.md](plid-chunk.md) |
| `PCHK` | プラグイン状態本体（zlib圧縮された全パラメータ） | [pchk-chunk.md](pchk-chunk.md) |

## RIFF ヘッダ

ファイル先頭 12 バイトが RIFF ヘッダとなる。

| Offset | Size | Content | Description |
|--------|------|---------|-------------|
| 0x00 | 4 | `"RIFF"` | RIFF マジック（ASCII） |
| 0x04 | 4 | riff_size | RIFF ペイロードサイズ（リトルエンディアン u32）。`ファイルサイズ - 8` に等しい |
| 0x08 | 4 | `"NIKS"` | フォーマット識別子（ASCII） |

`riff_size` はリトルエンディアンで格納され、実ファイルサイズとは `riff_size + 8 = ファイルサイズ` の関係にある。例として `Abandoned.nksf`（32,736 バイト）では先頭 8 バイトが `52 49 46 46 d8 7f 00 00` となり、`0x7fd8 + 8 = 32,736` で一致する。

`"RIFF"` でも `"NIKS"` でもないファイルはパーサーが拒否する（それぞれ `ParseError::InvalidRiff` / `ParseError::InvalidNiks`）。

> 実装: [`lib/src/riff_reader.rs`](../../lib/src/riff_reader.rs) の `RiffReader`

## チャンク共通ヘッダ

RIFF コンテナ内の各チャンクは、すべて次の共通ヘッダで始まる。

| Offset（チャンク先頭基準） | Size | Content | Description |
|--------|------|---------|-------------|
| +0x00 | 4 | chunk_id | チャンク ID（ASCII 4 文字。例: `"NISI"`） |
| +0x04 | 4 | chunk_size | チャンクデータ部のサイズ（リトルエンディアン u32） |
| +0x08 | 4 | version | チャンクバージョン（リトルエンディアン u32）。**観測値: 常に 1** |
| +0x0C | ... | payload | チャンク固有データ |

重要なポイント:

- **サイズ系フィールドはすべてリトルエンディアン**（RIFF 部）。一方、ペイロード内の MessagePack 整数はビッグエンディアンである（[messagepack-encoding.md](messagepack-encoding.md) の「エンディアンの注意」参照）。
- `version` が 1 以外のチャンクはパーサーがエラーとする。
- RIFF 規約に従い、`chunk_size` が奇数のチャンクの後ろには 1 バイトのパディングが置かれることがある。`RiffReader` はこれをスキップして次のチャンクへ進む。
- パーサーは不正な巨大サイズによるメモリ枯渇を防ぐため、`chunk_size` が 100MB を超えるチャンクを拒否する。

## チャンク一覧とレイアウト

.nksf ファイルは `NIKS` 識別子の直後に 4 つのチャンクをこの順で格納する。以下は `Abandoned.nksf`（32,736 バイト）での観測例である（サイズは概数、観測値）。

```
┌─────────────────────────────────────────────────────────┐
│ RIFF ヘッダ (12 B): "RIFF" + サイズ(LE) + "NIKS"          │
├─────────────────────────────────────────────────────────┤
│ NISI チャンク（約 349 B）                                 │
│   チャンクヘッダ 8 B + version 4 B + MessagePack 約 341 B │
├─────────────────────────────────────────────────────────┤
│ NICA チャンク（約 588 B）                                 │
│   チャンクヘッダ 8 B + version 4 B + MessagePack 約 580 B │
├─────────────────────────────────────────────────────────┤
│ PLID チャンク（20〜73 B）                                │
│   チャンクヘッダ 8 B + version 4 B + MessagePack         │
├─────────────────────────────────────────────────────────┤
│ PCHK チャンク（約 31,700 B）                              │
│   チャンクヘッダ 8 B + 固有ヘッダ 20 B                    │
│   + zlib圧縮データ（展開後 約 507 KB）                    │
└─────────────────────────────────────────────────────────┘
```

| # | チャンク ID | 目的 | ペイロードのエンコーディング |
|---|------------|------|------------------------------|
| 1 | `NISI` | プリセットのメタデータ。プリセット名、作者、ベンダー、バンクチェーン、キャラクター/モードタグ、UUID 等 | version(u32 LE) + MessagePack マップ |
| 2 | `NICA` | ホスト (DAW) 側に公開するパラメータのアサインメント。パラメータ ID・名前・フラグの一覧 | version(u32 LE) + MessagePack マップ |
| 3 | `PLID` | 生成元プラグインの識別情報。VST マジックナンバー、プラグイン名・ベンダー名（省略され得る） | version(u32 LE) + MessagePack マップ |
| 4 | `PCHK` | プラグイン状態本体。シンセの全パラメータ値・Performer データ等を zlib 圧縮した MessagePack ストリーム | version/固有ヘッダ + zlib圧縮 + MessagePack 連続ストリーム |

## 未知チャンクの扱い

パーサーは上記 4 チャンク以外のチャンク ID を検出した場合、そのファイルを不正なものとみなして `ParseError::UnknownChunk` を返す（無視して読み飛ばしたりはしない）。また、4 チャンクのいずれかが欠落している場合も `ParseError::InvalidNiks` となる。

さらに `parser.rs` は読み取った全バイトが消費されたことを `verify_complete()` で検証し、未解析のバイトが残っていれば `ParseError::IncompleteParse` を返す。これは「1 バイトも見逃さない」という本パーサーの設計方針（完全なバイト解析）に基づく。

> 実装: [`lib/src/parser.rs`](../../lib/src/parser.rs)

## 関連ドキュメント

- [NISI チャンク仕様](nisi-chunk.md)
- [NICA チャンク仕様](nica-chunk.md)
- [PLID チャンク仕様](plid-chunk.md)
- [PCHK チャンク仕様](pchk-chunk.md)
- [MessagePack エンコーディング](messagepack-encoding.md)
