# タスク一覧

## Phase 1: 基盤の構築

### 001: 依存関係の追加

libクレートに必要な依存関係を追加する。

Status: Completed

### 002: エラー型の定義

ParseErrorとその関連型を定義する (error.rs)。

Status: Completed

### 003: 基本的なデータ構造の定義

NksfFile、NisiMetadata、NicaData、Parameterなどの構造体を定義する (types.rs)。

Status: Completed

### 004: RIFFリーダーの実装

RIFFファイルの読み取りとチャンク解析を実装する (riff_reader.rs)。

Status: Completed

## Phase 2: MessagePackパーサーの実装

### 005: NISIチャンクパーサーの実装

NISIチャンクのMessagePackデータをデシリアライズする (nisi_parser.rs)。

Status: Completed

### 006: NICAチャンクパーサーの実装

NICAチャンクのMessagePackデータをデシリアライズする (nica_parser.rs)。

Status: Completed

## Phase 3: 統合パーサーの実装

### 007: メインパーサーAPIの実装

parse_nksf()およびparse_nksf_from_bytes()を実装する (parser.rs)。

Status: Planned

### 008: lib.rsの実装とモジュール構成

ライブラリエントリーポイントとモジュール構成を整える。

Status: Planned

### 009: Abandoned.nksfの統合テスト作成

最初の統合テストとしてAbandoned.nksfの解析テストを作成する。

Status: Planned

## Phase 4: CLIツールの実装

### 010: CLIツールの実装

コマンドライン引数処理とJSON出力を実装する (cli/src/main.rs)。

Status: Planned

### 011: CLI依存関係の追加

cliクレートに必要な依存関係を追加する。

Status: Planned

## Phase 5: エラーハンドリングと改善

### 012: バリデーション実装

フォーマット検証、バージョンチェック、必須フィールドの存在確認を実装する。

Status: Planned

### 013: エラーハンドリングの改善

エラーメッセージの改善と追加のエラーケース対応。

Status: Planned

## Phase 6: テストとドキュメント

### 014: 全プリセットの統合テスト作成

fixture/ディレクトリ内の全.nksfファイルに対する統合テストを作成する。

Status: Planned

### 015: 統合テストの拡充

integration.rsに複数ファイル処理やエラーケースのテストを追加する。

Status: Planned

### 016: ドキュメントコメントの追加

パブリックAPIに対するRustdocコメントを追加する。

Status: Planned

---

## 実装順序

1. 001 → 002 → 003 → 004 (Phase 1)
2. 005 → 006 (Phase 2)
3. 007 → 008 → 009 (Phase 3)
4. 011 → 010 (Phase 4)
5. 012 → 013 (Phase 5)
6. 014 → 015 → 016 (Phase 6)

## 品質基準

- 全テストケースがパスすること
- `cargo fmt` でフォーマットされていること
- `cargo clippy` で警告が出ないこと
- 全バイトを解析し、不明なデータを捨てないこと
