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

Status: Completed

### 008: lib.rsの実装とモジュール構成

ライブラリエントリーポイントとモジュール構成を整える。

Status: Completed

### 009: Abandoned.nksfの統合テスト作成

最初の統合テストとしてAbandoned.nksfの解析テストを作成する。

Status: Completed

### 009-001: PLIDチャンクの完全解析

PLIDチャンク（Plugin ID）のMessagePackデータを完全に解析し、構造化する。

Status: Completed

### 009-002: PCHKチャンクの完全解析

PCHKチャンク（Plugin Chunk）のバイナリフォーマットを完全に解析し、構造化する。

Status: Completed

### 009-003: PLIDパーサーの実装

PLIDチャンクパーサーを実装し、メインパーサーに統合する。

Status: Completed

### 009-004: PCHKパーサーの実装

PCHKチャンクパーサー（zlib展開 + MessagePackストリーム）を実装する。

Status: Completed

### 009-005: NksfFile構造の更新とテスト

NksfFile構造体を更新し、unknown_chunksからPLID/PCHKを削除、テストを更新する。

Status: Completed

## Phase 4: CLIツールの実装

### 010: CLI依存関係の追加

cliクレートに必要な依存関係を追加する。

Status: Completed

### 011: CLIツールの実装

コマンドライン引数処理とJSON出力を実装する (cli/src/main.rs)。

Status: Completed

## Phase 5: エラーハンドリングと改善

### 012: バリデーション実装（更新版）

フィールドレベルのバリデーションを追加する。
※ 基本的なバリデーション（フォーマット検証、バージョンチェック、セキュリティ対策）は実装済み。

Status: Completed

### 013: エラーハンドリングの改善（更新版）

CLIツールでのユーザーフレンドリーなエラーメッセージを実装する。
※ libクレートのエラーハンドリングは実装済み。タスク011に依存。

Status: Completed

## Phase 6: テストとドキュメント

### 014: 全プリセットの統合テスト作成

fixture/ディレクトリ内の全.nksfファイルに対する統合テストを作成する。

Status: Completed

### 015: 統合テストの拡充

integration.rsに複数ファイル処理やエラーケースのテストを追加する。

Status: Completed

### 016: ドキュメントコメントの追加

パブリックAPIに対するRustdocコメントを追加する。

Status: Completed

### 017: テストアーキテクチャの再設計

テスト用固定値とテスト実行コードを完全に分離し、全720プリセットに対する詳細テストを実現する。

Status: Completed

## Phase 7: テストツールの強化

### 018: テストライブラリの導入

テストの表現力、実行速度、保守性を向上させるため、Rustエコシステムの標準的なテストライブラリを導入する。

- 必須級: rstest, cargo-nextest, insta, pretty_assertions
- 強く推奨: proptest, criterion

Status: Completed

## Phase 8: CLI機能の拡充

### 019: CLI機能拡充

CLIツールのコマンドラインオプションを拡充し、複数ファイルの処理、出力先の指定、上書き動作の制御を実装する。

- 追加オプション: `-o,--output`, `-d,--dir`, `-v,--version`, `--overwrite`
- 複数ファイル入力対応

Status: Completed

### 020: JSONLサポート

CLIツールにJSON Lines (JSONL)形式のサポートを追加し、複数ファイルの効率的な処理を実現する。

- `--jsonl`フラグの追加
- `-c, --compact`オプションの削除
- 複数ファイルのJSONL結合出力対応

Status: Not Started

---

## 実装順序

1. 001 → 002 → 003 → 004 (Phase 1)
2. 005 → 006 (Phase 2)
3. 007 → 008 → 009 → 009-001 → 009-002 → 009-003 → 009-004 → 009-005 (Phase 3)
4. 010 → 011 (Phase 4)
5. 012 → 013 (Phase 5)
6. 014 → 015 → 016 → 017 (Phase 6)
7. 018 (Phase 7)
8. 019 → **020** (Phase 8)

## 品質基準

- 全テストケースがパスすること
- `cargo fmt` でフォーマットされていること
- `cargo clippy` で警告が出ないこと
- 全バイトを解析し、不明なデータを捨てないこと
