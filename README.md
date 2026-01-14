# nksf-parser

.nksf (Massive Xのプリセットファイル) のパーサーライブラリおよびCLIツール

## 概要

このプロジェクトは、Native Instruments Massive Xのプリセットファイル (.nksf) を解析し、Rust構造体およびJSON形式で扱えるようにするツールです。

## 機能

- **完全なバイト解析**: ファイルの全バイトを構造化して解析（データ欠損なし）
- **4つのチャンク対応**:
  - NISI: メタデータ（プリセット名、作者、タグ等）
  - NICA: パラメータアサインメント（マクロコントロール）
  - PLID: プラグインID（VSTマジックナンバー）
  - PCHK: プラグインチャンク（全パラメータ値、zlib圧縮）
- **全720個のMassive X Libraryプリセット対応**
- **セキュリティ対策**: メモリ枯渇攻撃、Zip Bomb攻撃への防御機能

## プロジェクト構成

- **lib** (`nksf-parser`): パーサーライブラリ
- **cli** (`nksf-parser-cli`): コマンドラインツール

## 使用方法

### ライブラリとして使用

```rust
use nksf_parser::parse_nksf;
use std::path::Path;

let path = Path::new("preset.nksf");
let nksf = parse_nksf(&path)?;

println!("Preset: {}", nksf.metadata.name);
println!("Author: {}", nksf.metadata.author);
println!("Parameters: {}", nksf.plugin_chunk.values.len());
```

### CLIツールとして使用

```bash
# JSON出力（整形あり）
cargo run -p nksf-parser-cli -- preset.nksf

# コンパクトJSON出力
cargo run -p nksf-parser-cli -- preset.nksf --compact

# ファイルに保存
cargo run -p nksf-parser-cli -- preset.nksf > output.json
```

## 開発

### ビルド

```bash
# 全体のビルド
cargo build

# リリースビルド
cargo build --release
```

### テスト

```bash
# 全テスト実行
cargo test

# ライブラリのテストのみ
cargo test -p nksf-parser

# パフォーマンステスト（通常は除外）
cargo test -p nksf-parser -- --ignored
```

### フォーマット・Lint

```bash
# コードフォーマット
cargo fmt

# Clippy（リンター）
cargo clippy
```

## テスト状況

- **ユニットテスト**: 32個
- **統合テスト**: 31個（全720プリセット対応確認済み）
- **ドキュメントテスト**: 3個
- **合計**: 66テスト、全て成功

## 技術仕様

### ファイルフォーマット

- **ベースフォーマット**: RIFF (Resource Interchange File Format)
- **フォーマット識別子**: "NIKS" (Native Instruments Komplete Sound)
- **エンコーディング**: MessagePack
- **圧縮**: zlib (PCHKチャンクのみ)

### セキュリティ対策

- チャンクサイズ上限: 100MB
- zlib展開後サイズ上限: 50MB
- MessagePack値の数上限: 100,000個

## ライセンス

[ライセンス情報]

## 参照

- [NKSF File Format (Community)](https://community.native-instruments.com/discussion/13469/)
- [Native Instruments Massive X](https://www.native-instruments.com/en/products/komplete/synths/massive-x/)
