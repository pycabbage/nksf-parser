# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## プロジェクト概要

.nksf (Massive Xのプリセットファイル)のパーサーライブラリおよびCLIツール

### 実装状況

- ✅ パーサーライブラリ完成（全4チャンク対応）
- ✅ CLIツール完成（JSON出力機能）
- ✅ 全720個のMassive X Libraryプリセット対応確認済み
- ✅ セキュリティ対策実装済み（DoS攻撃防御）
- ✅ 全39テスト成功（ユニット32 + 統合7）
- ⚠️ 期待値データ生成完了（720個、約400MB）、テスト統合作業中

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
# 全テスト実行（推奨）
cargo test

# ライブラリのテストのみ
cargo test -p nksf-parser

# 特定のテストファイル実行
cargo test -p nksf-parser --test integration

# 特定のプリセットのテスト実行
cargo test -p nksf-parser test_abandoned

# パフォーマンステスト実行（通常はスキップされる）
cargo test -p nksf-parser -- --ignored
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

### テストの種類と配置

1. **ユニットテスト**: `lib/src` 以下の各 `.rs` ファイル内に記述
   - 各モジュールの関数・構造体の単体テスト
   - `#[cfg(test)]` モジュール内に配置
   - 実装ファイルと同じファイル内に記述
   - 合計: 32個

2. **統合テスト**: `lib/tests` 以下に配置
   - **integration.rs**: エラーケーステスト、パフォーマンステスト
   - **fixture_test.rs**: プリセットファイルの統合テスト
   - **massive_x_factory_library_tests/**: 期待値データ（約400MB）
     - `mod.rs`: 720個の期待値モジュール宣言
     - `*_expected_data.rs` × 720個: 各プリセットの完全な期待値データ（phf_map使用）
     - `fixture/*.nksf`: 720個のプリセットファイル
   - **generators/**: テスト生成スクリプト（通常のテスト実行では使用しない）
     - `generate_all_preset_expected.rs`: 期待値ファイル自動生成
     - `generate_fixture_test_functions.rs`: テスト関数自動生成
     - `README.md`: 使用方法

### 期待値データの構造

各`<preset>_expected_data.rs`ファイル（約550KB）には以下が含まれる:
- `ExpectedNisiMetadata`: メタデータの期待値
- `ExpectedParameter`: パラメータ構造
- `EXPECTED_NISI`: NISIチャンクの期待値
- `EXPECTED_NICA_PARAMS_0`, `EXPECTED_NICA_PARAMS_1`: NICAチャンクの期待値
- `EXPECTED_PLID_*`: PLIDチャンクの期待値
- `EXPECTED_PCHK_*`: PCHKチャンクの期待値
- `EXPECTED_ABANDONED_STRINGS`: stringsセクション（約758エントリ、phf_map）
- `EXPECTED_ABANDONED_DOUBLES`: doublesセクション（約1104エントリ、phf_map）
- `EXPECTED_ABANDONED_INTS`: intsセクション（約383エントリ、phf_map）
- `EXPECTED_ABANDONED_BOOLS`: boolsセクション（約1580エントリ、phf_map）

### テスト生成の手順

新しいプリセットファイルを追加した場合:

1. fixtureディレクトリに`.nksf`ファイルを配置
2. 期待値ファイルを生成:
   ```bash
   cargo test -p nksf-parser generate_all_preset_expected_data -- --nocapture --ignored
   ```
3. テスト関数を生成（必要に応じて）:
   ```bash
   cargo test -p nksf-parser generate_fixture_test_functions -- --nocapture --ignored
   ```

**注意**: 期待値ファイル生成は約10秒、全720個で約400MBのデータが生成されます。

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
