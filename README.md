# nksf-parser

.nksf (Massive Xのプリセットファイル) のパーサーライブラリおよびCLIツール

## 概要

このプロジェクトは、Native Instruments Massive Xのプリセットファイル (.nksf) を解析し、Rust構造体およびJSON形式で扱えるようにするツールです。

### 特徴

- **完全なバイト解析**: ファイルの全バイトを構造化して解析（データ欠損なし）
- **4つのチャンク対応**: NISI（メタデータ）、NICA（パラメータ）、PLID（プラグインID）、PCHK（プラグインデータ）
- **全720個のMassive X Libraryプリセット対応**
- **セキュリティ対策**: メモリ枯渇攻撃、Zip Bomb攻撃への防御機能

## インストール

### バイナリのダウンロード（推奨）

[Releases](https://github.com/[username]/nksf-parser/releases)ページから、お使いのプラットフォーム向けのバイナリをダウンロードしてください。

### ソースからビルド

```bash
# リポジトリのクローン
git clone https://github.com/[username]/nksf-parser.git
cd nksf-parser

# リリースビルド
cargo build --release

# バイナリは target/release/nksf-parser-cli に生成されます
```

## 使い方

### CLIツール

**単一ファイル処理**:
```bash
# JSON出力（整形あり、stdoutへ出力）
nksf-parser-cli preset.nksf

# JSONL形式（コンパクト、1行）
nksf-parser-cli preset.nksf --jsonl

# ファイルに保存
nksf-parser-cli preset.nksf -o output.json

# ディレクトリ指定
nksf-parser-cli preset.nksf -d ./output
```

**複数ファイル処理**:
```bash
# 個別JSON出力（カレントディレクトリ）
nksf-parser-cli preset1.nksf preset2.nksf preset3.nksf

# 個別JSON出力（ディレクトリ指定）
nksf-parser-cli -d ./output *.nksf

# JSONL結合出力（ファイル指定）
nksf-parser-cli -o combined.jsonl preset1.nksf preset2.nksf

# JSONL自動命名（カレント）
nksf-parser-cli preset1.nksf preset2.nksf preset3.nksf --jsonl
# → preset1_3files.jsonl

# JSONL自動命名（ディレクトリ指定）
nksf-parser-cli -d ./output *.nksf --jsonl
# → ./output/preset1_3files.jsonl
```

**その他**:
```bash
# 既存ファイルを上書き
nksf-parser-cli preset.nksf -o output.json --overwrite

# バージョン表示
nksf-parser-cli --version
```

### ライブラリとして使用

`Cargo.toml`に以下を追加:

```toml
[dependencies]
nksf-parser = "0.1.0"
```

コード例:

```rust
use nksf_parser::parse_nksf;
use std::path::Path;

let path = Path::new("preset.nksf");
let nksf = parse_nksf(&path)?;

println!("Preset: {}", nksf.metadata.name);
println!("Author: {}", nksf.metadata.author);
println!("Parameters: {}", nksf.plugin_chunk.values.len());
```

## 開発

開発者向けの詳細情報は[DEVELOPMENT.md](./DEVELOPMENT.md)を参照してください。

## ライセンス

[ライセンス情報]

## 参照

- [NKSF File Format (Community)](https://community.native-instruments.com/discussion/13469/)
- [Native Instruments Massive X](https://www.native-instruments.com/en/products/komplete/synths/massive-x/)
