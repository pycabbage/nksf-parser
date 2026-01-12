# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## プロジェクト概要

.nksf (Massive Xのプリセットファイル)のパーサーライブラリおよびCLIツール

### アーキテクチャ

本プロジェクトはRustワークスペース構成で、2つの主要コンポーネントから成る:

- **lib** (`nksf-parser`): .nksfファイルを解析するパーサーライブラリ
  - 責任分離の観点から、JSON形式の出力は行わない
  - JSON最適化も行わない（純粋なパーサー機能のみ）
- **cli** (`nksf-parser-cli`): コマンドラインインターフェース
  - libを使用して.nksfファイルを解析
  - 解析結果をJSON形式で出力
  - libへの依存関係が設定済み

## 開発コマンド

### ビルド
```bash
# ワークスペース全体のビルド
cargo build

# リリースビルド
cargo build --release

# 特定のクレートのビルド
cargo build -p nksf-parser       # ライブラリのみ
cargo build -p nksf-parser-cli   # CLIのみ
```

### テスト
```bash
# 全テスト実行
cargo test

# ライブラリのテストのみ
cargo test -p nksf-parser

# 特定のテストファイル実行
cargo test -p nksf-parser --test massive_x_factory_library_tests

# 特定のテスト関数実行
cargo test -p nksf-parser test_in_mod
```

### フォーマット・Lint
```bash
# コードフォーマット
cargo fmt

# フォーマットチェック（CI用）
cargo fmt --check

# Clippy（リンター）
cargo clippy

# Clippy（警告をエラーとして扱う）
cargo clippy -- -D warnings
```

### 依存関係の管理
```bash
# 依存関係の追加（Cargo.tomlを直接編集してはいけない）
cd lib && cargo add <package-name>
cd cli && cargo add <package-name>

# 開発依存関係の追加
cargo add --dev <package-name>
```

## テスト構造

### テストファイルの配置

- テストデータ: `lib/tests/massive_x_factory_library_tests/fixture/*.nksf`
- テストコード: `lib/tests/massive_x_factory_library_tests/*_test.rs`
- モジュール定義: `lib/tests/massive_x_factory_library_tests/mod.rs`

### 新しいテストの追加手順

1. `lib/tests/massive_x_factory_library_tests/` に `<preset_name>_test.rs` を作成
2. `lib/tests/massive_x_factory_library_tests/mod.rs` に `mod <preset_name>_test;` を追加
3. 対応する `.nksf` ファイルが `fixture/` ディレクトリに存在することを確認

例:
```rust
// lib/tests/massive_x_factory_library_tests/abandoned_test.rs
#[test]
fn test_in_mod() {
    // fixture/Abandoned.nksf を使ったテスト
    assert_eq!(1, 1);
}
```

## コーディング規約

- **Rustの標準スタイル**: rustfmt に従う
- **関数・構造体・ファイルの適切な分割**: 責任を明確に分離
- **ドキュメントコメント**: パブリックAPIには必ず記述
- **作業前の確認**: 必ず `pwd` でカレントディレクトリを確認

## 重要な制約

- ライブラリ (`lib`) では JSON 出力を実装しない
- CLI (`cli`) でのみ JSON 形式での出力を実装
- すべてのプリセットに対してテストを記述し、必ずパスさせる
