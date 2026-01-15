# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## プロジェクト概要

.nksf (Massive Xのプリセットファイル)のパーサーライブラリおよびCLIツール

### 実装状況

- ✅ パーサーライブラリ完成（全4チャンク対応）
- ✅ CLIツール完成（JSON出力機能）
- ✅ 全720個のMassive X Libraryプリセット対応確認済み
- ✅ セキュリティ対策実装済み（DoS攻撃防御）
- ✅ テストライブラリ導入完了（rstest, cargo-nextest, insta, pretty_assertions, proptest, criterion）
- ✅ 全741テスト成功（ユニット32 + 統合9 + fixture 720）

### アーキテクチャ

本プロジェクトはRustワークスペース構成で、2つの主要コンポーネントから成る:

- **lib** (`nksf-parser`): .nksfファイルを解析するパーサーライブラリ
  - 責任分離の観点から、JSON形式の出力は行わない
  - JSON最適化も行わない（純粋なパーサー機能のみ）
  - 全4チャンク対応: NISI（メタデータ）、NICA（パラメータ）、PLID（プラグインID）、PCHK（プラグインデータ）
  - セキュリティ対策: チャンクサイズ上限（100MB）、zlib展開サイズ上限（50MB）
- **cli** (`nksf-parser-cli`): コマンドラインインターフェース
  - libを使用して.nksfファイルを解析
  - 解析結果をJSON形式で出力
  - `--compact`フラグでコンパクトJSON出力対応
  - ユーザーフレンドリーなエラーメッセージ

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
# 全テスト実行（cargo-nextest使用、推奨）
cargo nextest run

# 従来のテストランナー使用
cargo test

# ライブラリのテストのみ
cargo nextest run -p nksf-parser

# 特定のテストファイル実行
cargo nextest run --test integration
cargo nextest run --test fixture_test  # 720プリセット

# プロパティベーステスト実行
cargo test --test integration test_parser_never_panics

# スナップショットテスト（更新モード）
INSTA_UPDATE=always cargo test --test fixture_test

# ベンチマーク実行
cargo bench
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

# 現在の開発依存関係
# - rstest: パラメータ化テスト・フィクスチャ
# - cargo-nextest: 高速テストランナー
# - insta: スナップショットテスト
# - pretty_assertions: アサーション強化
# - proptest: プロパティベーステスト
# - criterion: ベンチマーク
```

## テスト構造

### テストの種類と配置

1. **ユニットテスト**: `lib/src` 以下の各 `.rs` ファイル内に記述
   - 各モジュールの関数・構造体の単体テスト
   - `#[cfg(test)]` モジュール内に配置
   - 実装ファイルと同じファイル内に記述
   - 合計: 32個

2. **統合テスト**: `lib/tests` 以下に配置
   - **integration.rs**: エラーケーステスト、プロパティベーステスト
   - **fixture_test.rs**: 720プリセットの統合テスト（rstest + insta）
     - 720個の`#[case]`を1つの関数で実行
   - **snapshots/**: instaスナップショットファイル（約380MB）
     - `fixture_test__<Preset Name>.snap` × 720個: 各プリセットの期待値（YAML形式）
   - **massive_x_factory_library_tests/fixture/**: テストフィクスチャ
     - `*.nksf` × 720個: 全Massive X Libraryプリセット

3. **ベンチマーク**: `lib/benches` に配置
   - **parser_bench.rs**: パース処理のベンチマーク（criterion）

### テスト技術スタック

- **rstest**: パラメータ化テスト（`#[case]`で720プリセットを1関数で実行）
- **insta**: スナップショットテスト（YAMLで期待値を自動管理）
- **proptest**: プロパティベーステスト（任意入力でパニックしないことを検証）
- **criterion**: 統計的に正確なベンチマーク
- **pretty_assertions**: テスト失敗時のdiff表示
- **cargo-nextest**: 高速並列テスト実行

### スナップショットテストの管理

新しいプリセットファイルを追加した場合:

1. fixtureディレクトリに`.nksf`ファイルを配置
2. `lib/tests/fixture_test.rs`に`#[case("New Preset")]`を追加
3. スナップショットを生成・承認:
   ```bash
   INSTA_UPDATE=always cargo test --test fixture_test
   ```

**注意**: スナップショットファイルは全てGitにコミットされます（約380MB）。

## コーディング規約

- **Rustの標準スタイル**: rustfmt に従う
- **関数・構造体・ファイルの適切な分割**: 責任を明確に分離
- **ドキュメントコメント**: パブリックAPIには必ず記述
- **作業前の確認**: 必ず `pwd` でカレントディレクトリを確認

## 重要な制約

### 責任分離

- ライブラリ (`lib`) では JSON 出力を実装しない
- CLI (`cli`) でのみ JSON 形式での出力を実装

### パース要件

- **完全なバイト解析**: .nksfファイルの全てのバイトを解析する
- **データの欠損禁止**: 1バイトも見逃さず、「不明なデータ」として捨てることを許可しない
- すべてのチャンクとフィールドを構造化して保持する
- 解析できないデータがある場合はエラーを返す（無視しない）

### テスト要件

- すべてのプリセットに対してテストを記述し、必ずパスさせる
- `lib/src` 以下の各 `.rs` ファイルにユニットテストを記述する
- `lib/tests` 以下に統合テストを記述する

**現在の状況**:
- ユニットテスト: 32個実装済み ✅
- 統合テスト: 39個実装済み ✅
- 全720プリセットの期待値データ生成完了（約400MB）✅
- 全720プリセットの解析確認済み ✅
