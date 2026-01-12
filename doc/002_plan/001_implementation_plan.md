# .nksfパーサー実装計画

## 実装日時
2026-01-13

## 概要

本ドキュメントは、.nksfファイルパーサーライブラリ（@lib）およびCLIツール（@cli）の実装計画を定義する。

## アーキテクチャ設計

### レイヤー構造

```
┌─────────────────────────────────────┐
│   CLI Layer (@cli)                  │
│   - JSON出力                        │
│   - コマンドライン引数処理           │
└─────────────────┬───────────────────┘
                  │
┌─────────────────▼───────────────────┐
│   Parser Library (@lib)             │
│   - RIFFファイル解析                │
│   - MessagePackデシリアライズ       │
│   - データ構造定義                   │
└─────────────────────────────────────┘
```

### 責任分離

- **@lib**: 純粋なパーサー機能のみ
  - RIFFチャンクの読み取り
  - MessagePackデータのデシリアライズ
  - Rust構造体への変換
  - **JSON出力は行わない**（重要）

- **@cli**: ユーザーインターフェースとJSON出力
  - コマンドライン引数の処理
  - @libを使用した解析実行
  - **JSON形式での出力**

## 実装フェーズ

### フェーズ1: 基盤の構築

#### 1.1 依存関係の追加

```bash
cd lib
cargo add riff
cargo add rmp
cargo add rmp-serde
cargo add serde --features derive
cargo add thiserror  # エラーハンドリング用

cd ../cli
cargo add serde_json
cargo add clap --features derive  # CLI引数パース用
```

#### 1.2 基本的なデータ構造の定義

`lib/src/types.rs` を作成:

- `NksfFile`: トップレベル構造体
- `NisiMetadata`: メタデータ（NISIチャンク）
- `NicaData`: パラメータデータ（NICAチャンク）
- `Parameter`: パラメータ情報
- エラー型: `ParseError`

#### 1.3 RIFFリーダーの実装

`lib/src/riff_reader.rs` を作成:

- RIFFファイルのオープン
- チャンク列挙
- チャンクデータの読み取り
- "NIKS" フォーマット検証

### フェーズ2: MessagePackパーサーの実装

#### 2.1 NISIチャンクパーサー

`lib/src/nisi_parser.rs` を作成:

- バージョン情報の読み取り
- MessagePackデータのデシリアライズ
- `NisiMetadata` への変換

#### 2.2 NICAチャンクパーサー

`lib/src/nica_parser.rs` を作成:

- バージョン情報の読み取り
- MessagePackデータのデシリアライズ
- `NicaData` および `Parameter` への変換

### フェーズ3: 統合パーサーの実装

#### 3.1 メインパーサーAPI

`lib/src/parser.rs` を作成:

```rust
pub fn parse_nksf(path: &Path) -> Result<NksfFile, ParseError>;
pub fn parse_nksf_from_bytes(data: &[u8]) -> Result<NksfFile, ParseError>;
```

#### 3.2 テストの追加

各プリセットファイルに対するテスト:

- `lib/tests/massive_x_factory_library_tests/abandoned_test.rs`
  - Abandoned.nksfの解析テスト
  - メタデータの検証
  - パラメータの検証

テストファイルの追加時は `mod.rs` にモジュールを追加する必要がある。

### フェーズ4: CLIツールの実装

#### 4.1 CLI構造

`cli/src/main.rs`:

```rust
use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Input .nksf file path
    input: PathBuf,

    /// Output format (default: pretty JSON)
    #[arg(short, long)]
    compact: bool,
}
```

#### 4.2 JSON出力実装

- `nksf_parser::parse_nksf()` を呼び出し
- 結果を `serde_json::to_string_pretty()` でJSON化
- 標準出力に出力

### フェーズ5: エラーハンドリングと改善

#### 5.1 エラー型の定義

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Invalid RIFF format")]
    InvalidRiff,

    #[error("Invalid NIKS format")]
    InvalidNiks,

    #[error("MessagePack deserialization error: {0}")]
    MessagePackError(#[from] rmp_serde::decode::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Unknown chunk: {0}")]
    UnknownChunk(String),
}
```

#### 5.2 バリデーション

- フォーマット検証
- バージョンチェック
- 必須フィールドの存在確認

### フェーズ6: テストとドキュメント

#### 6.1 全プリセットのテスト作成

fixture/ ディレクトリ内の全.nksfファイルに対してテストを作成:

1. ファイルが正常に読み込めることを確認
2. 基本的なメタデータ（name, author, vendorなど）の存在確認
3. パラメータ配列が空でないことを確認

#### 6.2 統合テスト

`lib/tests/integration.rs`:

- 複数ファイルの連続解析
- エラーケースのテスト
- パフォーマンステスト（大量ファイル処理）

#### 6.3 ドキュメントコメントの追加

- パブリックAPIに対するRustdoc
- 使用例の記載
- エラーケースの説明

## ファイル構造

```
nksf-parser/
├── lib/
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs              # ライブラリエントリーポイント
│   │   ├── types.rs            # データ構造定義
│   │   ├── error.rs            # エラー型定義
│   │   ├── riff_reader.rs      # RIFFリーダー
│   │   ├── nisi_parser.rs      # NISIチャンクパーサー
│   │   ├── nica_parser.rs      # NICAチャンクパーサー
│   │   └── parser.rs           # メインパーサーAPI
│   └── tests/
│       ├── integration.rs
│       └── massive_x_factory_library_tests/
│           ├── mod.rs
│           ├── abandoned_test.rs
│           ├── alien_contact_test.rs
│           └── ... (他のプリセット)
├── cli/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs             # CLIエントリーポイント
└── doc/
    ├── 001_research/
    │   ├── 001_nksf_file_structure.md
    │   └── 002_messagepack_analysis.md
    └── 002_plan/
        └── 001_implementation_plan.md
```

## 実装順序

1. **フェーズ1**: 基盤構築（依存関係、基本構造）
2. **フェーズ2**: MessagePackパーサー実装
3. **フェーズ3**: 統合パーサー実装
4. **テスト**: Abandoned.nksfでの動作確認
5. **フェーズ4**: CLIツール実装
6. **フェーズ5**: エラーハンドリング
7. **フェーズ6**: 全プリセットのテスト作成

## 品質基準

### テスト

- 全テストケースがパスすること
- 全プリセットファイルが正常に解析できること
- エラーケースが適切にハンドリングされること

### コードスタイル

- `cargo fmt` でフォーマットされていること
- `cargo clippy` で警告が出ないこと
- ドキュメントコメントが適切に記述されていること

### パフォーマンス

- 1ファイルあたり10ms以内で解析できること（目安）
- メモリ使用量が適切であること

## 注意事項

### 既知の未解析部分

以下の部分は現時点では完全に理解されていない:

1. `__ni_internal` フィールドの詳細構造
2. NICAチャンクの第2要素（追加データ）
3. 将来的なバージョン変更への対応

これらは段階的に解析し、必要に応じて構造を拡張する。

### 将来の拡張

1. 他のチャンク（存在する場合）のサポート
2. .nksfファイルの書き込み機能
3. プリセットの編集機能
4. 他のNative Instrumentsフォーマットのサポート

## 参照ドキュメント

- [001_nksf_file_structure.md](../001_research/001_nksf_file_structure.md)
- [002_messagepack_analysis.md](../001_research/002_messagepack_analysis.md)
- [INSTRUCT.md](../../INSTRUCT.md)
- [CLAUDE.md](../../CLAUDE.md)
