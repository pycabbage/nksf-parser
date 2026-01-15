# 開発ガイド

このドキュメントは、nksf-parserプロジェクトの開発者向けガイドです。

## 目次

- [プロジェクト構成](#プロジェクト構成)
- [開発環境のセットアップ](#開発環境のセットアップ)
- [開発コマンド](#開発コマンド)
- [テスト構造](#テスト構造)
- [コーディング規約](#コーディング規約)
- [重要な制約](#重要な制約)
- [技術仕様](#技術仕様)
- [コントリビューション](#コントリビューション)

## プロジェクト構成

本プロジェクトはRustワークスペース構成で、2つの主要コンポーネントから成ります:

```
nksf-parser/
├── lib/                    # nksf-parser ライブラリ
│   ├── src/
│   │   ├── lib.rs         # ライブラリエントリポイント
│   │   ├── parser.rs      # メインパーサーロジック
│   │   ├── chunks/        # チャンク解析モジュール
│   │   └── ...
│   └── tests/             # 統合テスト
│       ├── integration.rs
│       ├── fixture_test.rs
│       ├── generators/    # テスト生成スクリプト
│       └── massive_x_factory_library_tests/
├── cli/                    # nksf-parser-cli CLIツール
│   └── src/
│       └── main.rs
└── Cargo.toml             # ワークスペース設定
```

### コンポーネントの責任

#### ライブラリ (`lib/nksf-parser`)

- .nksfファイルの解析
- Rust構造体へのデータマッピング
- **責任分離**: JSON形式の出力は行わない
- **データの完全性**: すべてのバイトを構造化して解析

対応チャンク:
- **NISI**: メタデータ（プリセット名、作者、タグ等）
- **NICA**: パラメータアサインメント（マクロコントロール）
- **PLID**: プラグインID（VSTマジックナンバー）
- **PCHK**: プラグインチャンク（全パラメータ値、zlib圧縮）

#### CLIツール (`cli/nksf-parser-cli`)

- コマンドラインインターフェース
- ライブラリを使用した.nksfファイルの解析
- JSON形式での出力
- `--compact`フラグでコンパクトJSON出力
- ユーザーフレンドリーなエラーメッセージ

## 開発環境のセットアップ

### 必要なツール

- Rust 1.70以上（推奨: 最新のstable版）
- Cargo（Rustに同梱）

### セットアップ手順

1. リポジトリのクローン:
   ```bash
   git clone https://github.com/[username]/nksf-parser.git
   cd nksf-parser
   ```

2. 依存関係の確認:
   ```bash
   cargo check
   ```

3. ビルドの確認:
   ```bash
   cargo build
   ```

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

# 詳細な出力でテスト実行
cargo test -- --nocapture
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

#### 1. ユニットテスト（32個）

- **配置**: `lib/src` 以下の各 `.rs` ファイル内
- **目的**: 各モジュールの関数・構造体の単体テスト
- **マーク**: `#[cfg(test)]` モジュール内に配置
- **特徴**: 実装ファイルと同じファイル内に記述

#### 2. 統合テスト（31個 + 720プリセット）

- **配置**: `lib/tests` 以下
- **構成**:
  - `integration.rs`: エラーケーステスト、パフォーマンステスト
  - `fixture_test.rs`: プリセットファイルの統合テスト
  - `massive_x_factory_library_tests/`: 期待値データ（約400MB）
    - `mod.rs`: 720個の期待値モジュール宣言
    - `*_expected_data.rs` × 720個: 各プリセットの完全な期待値データ（phf_map使用）
    - `fixture/*.nksf`: 720個のプリセットファイル
  - `generators/`: テスト生成スクリプト（通常のテスト実行では使用しない）
    - `generate_all_preset_expected.rs`: 期待値ファイル自動生成
    - `generate_fixture_test_functions.rs`: テスト関数自動生成
    - `README.md`: 使用方法

### テスト状況

- **ユニットテスト**: 32個 ✅
- **統合テスト**: 39個 ✅
- **全テスト**: 成功
- **対応プリセット**: 全720個の期待値データ生成完了（約400MB）

### 期待値データの構造

各`<preset>_expected_data.rs`ファイル（約550KB）には以下が含まれます:

- `ExpectedNisiMetadata`: メタデータの期待値
- `ExpectedParameter`: パラメータ構造
- `EXPECTED_NISI`: NISIチャンクの期待値
- `EXPECTED_NICA_PARAMS_0`, `EXPECTED_NICA_PARAMS_1`: NICAチャンクの期待値
- `EXPECTED_PLID_*`: PLIDチャンクの期待値
- `EXPECTED_PCHK_*`: PCHKチャンクの期待値
- `EXPECTED_<PRESET>_STRINGS`: stringsセクション（約758エントリ、phf_map）
- `EXPECTED_<PRESET>_DOUBLES`: doublesセクション（約1104エントリ、phf_map）
- `EXPECTED_<PRESET>_INTS`: intsセクション（約383エントリ、phf_map）
- `EXPECTED_<PRESET>_BOOLS`: boolsセクション（約1580エントリ、phf_map）

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

### 基本原則

- **YAGNI**: 将来使うかもしれない機能は実装しない
- **DRY**: 重複コードは必ず関数化・モジュール化する
- **KISS**: 複雑な解決策より単純な解決策を優先

### Rustスタイル

- **フォーマット**: `rustfmt`に従う
- **命名規則**: Rustの標準命名規則に準拠
  - スネークケース: 関数、変数、モジュール
  - キャメルケース: 型、トレイト
  - スクリーミングスネークケース: 定数
- **関数・構造体・ファイルの適切な分割**: 責任を明確に分離
- **ドキュメントコメント**: パブリックAPIには必ず記述
  - `///` を使用してドキュメントコメントを記述
  - 例を含めることを推奨

### ファイル構成

- **末尾改行**: すべてのファイルの末尾に改行を入れる（trailing newline）
- **コメント**: コード内コメントは日本語で記述
- **インポート順序**: std → 外部クレート → 内部モジュール

## 重要な制約

### 責任分離

- ライブラリ (`lib`) では **JSON出力を実装しない**
- CLI (`cli`) でのみ JSON形式での出力を実装
- ライブラリは純粋なパーサー機能のみを提供

### パース要件

- **完全なバイト解析**: .nksfファイルのすべてのバイトを解析する
- **データの欠損禁止**: 1バイトも見逃さず、「不明なデータ」として捨てることを許可しない
- すべてのチャンクとフィールドを構造化して保持する
- 解析できないデータがある場合はエラーを返す（無視しない）

### テスト要件

- すべてのプリセットに対してテストを記述し、必ずパスさせる
- `lib/src` 以下の各 `.rs` ファイルにユニットテストを記述する
- `lib/tests` 以下に統合テストを記述する

### セキュリティ要件

- チャンクサイズ上限: 100MB
- zlib展開サイズ上限: 50MB
- MessagePack値の数上限: 100,000個
- DoS攻撃、Zip Bomb攻撃への防御を実装

## 技術仕様

### ファイルフォーマット

- **ベースフォーマット**: RIFF (Resource Interchange File Format)
- **フォーマット識別子**: "NIKS" (Native Instruments Komplete Sound)
- **エンコーディング**: MessagePack
- **圧縮**: zlib (PCHKチャンクのみ)

### チャンク構造

各チャンクは以下の構造を持ちます:

```
[4 bytes] チャンクID (例: "NISI", "NICA", "PLID", "PCHK")
[4 bytes] チャンクサイズ (リトルエンディアン)
[n bytes] チャンクデータ
```

### MessagePack構造

各チャンクのデータはMessagePack形式でエンコードされています:

- **NISI**: Map形式（メタデータのキー・バリューペア）
- **NICA**: Array of Maps（パラメータ配列）
- **PLID**: Array（プラグインID配列）
- **PCHK**: Map形式（zlib圧縮後、パラメータのキー・バリューペア）

## コントリビューション

### 作業開始時

1. 作業前に必ず公式ドキュメントを調査する
2. `pwd`でカレントディレクトリを確認する
3. 実装前にライブラリの型定義と公式ドキュメントを確認する

### 作業終了時

1. テストを実行:
   ```bash
   cargo test
   ```

2. フォーマット・Lintを実行:
   ```bash
   cargo fmt
   cargo clippy
   ```

3. エビデンスを残す（修正の根拠を明確にする）

4. プロジェクトの状況に変化があった場合、`CLAUDE.md`を更新する

### プルリクエスト

- 明確なタイトルと説明を記述
- 変更内容の概要を説明
- テスト結果を含める
- 関連するIssueがあれば参照

## 参考資料

- [NKSF File Format (Community)](https://community.native-instruments.com/discussion/13469/)
- [Native Instruments Massive X](https://www.native-instruments.com/en/products/komplete/synths/massive-x/)
- [RIFF Format Specification](https://en.wikipedia.org/wiki/Resource_Interchange_File_Format)
- [MessagePack Specification](https://msgpack.org/)
