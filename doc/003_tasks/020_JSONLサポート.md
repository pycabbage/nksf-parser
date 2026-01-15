# タスク020: JSONLサポート

## 概要

CLIツールにJSON Lines (JSONL)形式のサポートを追加し、複数ファイルの効率的な処理を実現する。

## 目的

- JSONL形式での出力対応
- 複数ファイルの結合出力
- ストリーミング処理への対応

## タスク詳細

### オプションの変更

#### 削除するオプション
- `-c, --compact` オプション（JSONLに置き換え）

#### 追加するオプション
- `--jsonl` フラグ: JSONL形式で出力

### 動作仕様

#### 単一ファイル指定時

**デフォルト（通常のJSON）**:
```bash
nksf-parser-cli preset.nksf
# → 整形されたJSON（pretty-print）をstdoutに出力
```

**JSONL形式**:
```bash
nksf-parser-cli preset.nksf --jsonl
# → 1行のコンパクトJSONをstdoutに出力
```

**ファイル出力**:
```bash
# 通常のJSON（pretty-print）
nksf-parser-cli preset.nksf -o output.json

# JSONL形式
nksf-parser-cli preset.nksf -o output.jsonl --jsonl
```

#### 複数ファイル指定時

**1. `-o/--output` 指定時: 全てを1つのJSONLファイルに結合**
```bash
nksf-parser-cli preset1.nksf preset2.nksf preset3.nksf -o output.jsonl
# → output.jsonlに3行のJSONL形式で出力
# 各行が1つのプリセットに対応
```

**2. `-o/--output` なし `-d/--dir` のみ: 個別JSON出力**
```bash
nksf-parser-cli -d ./output preset1.nksf preset2.nksf preset3.nksf
# → ./output/preset1.json（整形あり）
# → ./output/preset2.json（整形あり）
# → ./output/preset3.json（整形あり）
```

**3. `-d/--dir` のみ + `--jsonl`: 自動命名でJSONL結合**
```bash
nksf-parser-cli -d ./output preset1.nksf preset2.nksf preset3.nksf --jsonl
# → ./output/preset1_3files.jsonl（3行のJSONL）
# ファイル名: 最初のファイル名 + ファイル数
```

### JSONL形式の仕様

JSON Lines (JSONL):
- 各行が1つの完全なJSONオブジェクト
- 改行文字（`\n`）で区切られる
- コンパクトJSON（pretty-printなし）

**例**:
```jsonl
{"metadata":{"name":"Abandoned",...},"parameters":{...}}
{"metadata":{"name":"Alien Contact",...},"parameters":{...}}
{"metadata":{"name":"All Rise",...},"parameters":{...}}
```

### ファイル名生成規則（JSONL自動命名時）

**パターン**: `{first_file_stem}_{count}files.jsonl`

**例**:
- 入力: `preset1.nksf preset2.nksf preset3.nksf`
- 出力: `preset1_3files.jsonl`

- 入力: `/path/to/Abandoned.nksf /path/to/Alien Contact.nksf`
- 出力: `Abandoned_2files.jsonl`

## 実装手順

### ステップ1: Args構造体の更新

```rust
#[derive(Parser)]
#[command(name = "nksf-parser-cli", version, about = "Parse .nksf (Massive X preset) files and output as JSON")]
struct Args {
    /// Input .nksf file(s) to parse
    #[arg(required = true)]
    input: Vec<PathBuf>,

    /// Output file (default: stdout for single file)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Output directory (default: current directory)
    #[arg(short, long)]
    dir: Option<PathBuf>,

    /// Output in JSON Lines format (one JSON object per line)
    #[arg(long)]
    jsonl: bool,

    /// Overwrite existing files without asking
    #[arg(long)]
    overwrite: bool,
}
```

### ステップ2: シリアライズ関数の更新

```rust
/// JSON文字列にシリアライズ
fn serialize_json(nksf: &nksf_parser::NksfFile) -> Result<String, Box<dyn Error>> {
    // JSONLは常にコンパクト（1行）
    let json = serde_json::to_string(nksf)?;
    Ok(json)
}

/// 整形されたJSON文字列にシリアライズ
fn serialize_json_pretty(nksf: &nksf_parser::NksfFile) -> Result<String, Box<dyn Error>> {
    let json = serde_json::to_string_pretty(nksf)?;
    Ok(json)
}
```

### ステップ3: 単一ファイル処理の更新

```rust
fn process_single_file(
    input: &Path,
    output: Option<&Path>,
    dir: Option<&Path>,
    jsonl: bool,
    overwrite: bool,
) -> Result<(), Box<dyn Error>> {
    let nksf = parse_with_error_hints(input)?;

    // JSONL指定時はコンパクト、それ以外は整形あり
    let json_output = if jsonl {
        serialize_json(&nksf)?
    } else {
        serialize_json_pretty(&nksf)?
    };

    // 出力処理（既存と同じ）
    // ...
}
```

### ステップ4: 複数ファイル処理の更新

```rust
fn process_multiple_files(
    inputs: &[PathBuf],
    output: Option<&Path>,
    dir: Option<&Path>,
    jsonl: bool,
    overwrite: bool,
) -> Result<(), Box<dyn Error>> {
    match (output, dir, jsonl) {
        // Case 1: -o指定時 → 常にJSONL結合
        (Some(output_file), _, _) => {
            process_multiple_to_jsonl(inputs, output_file, overwrite)?;
        }
        // Case 2: -dのみ + --jsonl → 自動命名でJSONL結合
        (None, Some(output_dir), true) => {
            verify_directory_exists(output_dir);
            let output_file = generate_jsonl_filename(&inputs[0], inputs.len(), output_dir);
            process_multiple_to_jsonl(inputs, &output_file, overwrite)?;
        }
        // Case 3: -dのみ（--jsonlなし） → 個別JSON出力
        (None, dir, false) => {
            let output_dir = dir.unwrap_or_else(|| Path::new("."));
            verify_directory_exists(output_dir);
            process_multiple_to_individual_files(inputs, output_dir, overwrite)?;
        }
        // Case 4: -dなし --jsonl → カレントディレクトリに自動命名でJSONL
        (None, None, true) => {
            let output_file = generate_jsonl_filename(&inputs[0], inputs.len(), Path::new("."));
            process_multiple_to_jsonl(inputs, &output_file, overwrite)?;
        }
    }

    Ok(())
}
```

### ステップ5: JSONL処理関数の実装

```rust
/// 複数ファイルを1つのJSONLファイルに結合
fn process_multiple_to_jsonl(
    inputs: &[PathBuf],
    output_file: &Path,
    overwrite: bool,
) -> Result<(), Box<dyn Error>> {
    if output_file.exists() && !overwrite {
        eprintln!("Error: Output file '{}' already exists", output_file.display());
        eprintln!("Hint: Use --overwrite to overwrite existing files");
        process::exit(1);
    }

    let mut lines = Vec::new();

    for input in inputs {
        let nksf = parse_with_error_hints(input)?;
        let json_line = serialize_json(&nksf)?;
        lines.push(json_line);
    }

    let jsonl_content = lines.join("\n");
    std::fs::write(output_file, jsonl_content)?;

    eprintln!("✓ Processed {} files -> {}", inputs.len(), output_file.display());
    Ok(())
}

/// 複数ファイルを個別のJSONファイルに出力
fn process_multiple_to_individual_files(
    inputs: &[PathBuf],
    output_dir: &Path,
    overwrite: bool,
) -> Result<(), Box<dyn Error>> {
    for input in inputs {
        let nksf = parse_with_error_hints(input)?;
        let json_output = serialize_json_pretty(&nksf)?;

        let output_file = generate_output_path(input, output_dir);
        write_to_file(&output_file, &json_output, overwrite)?;

        eprintln!("✓ Processed: {} -> {}", input.display(), output_file.display());
    }

    Ok(())
}

/// JSONL自動命名時のファイル名を生成
fn generate_jsonl_filename(first_input: &Path, count: usize, output_dir: &Path) -> PathBuf {
    let file_stem = first_input.file_stem().unwrap_or_default();
    output_dir.join(format!("{}_{count}files.jsonl", file_stem.to_string_lossy()))
}
```

## 完了条件

- [ ] `--jsonl`フラグが実装されている
- [ ] `-c, --compact`オプションが削除されている
- [ ] 単一ファイル + デフォルト（pretty-print JSON）が動作する
- [ ] 単一ファイル + `--jsonl`（コンパクトJSON）が動作する
- [ ] 単一ファイル + `-o` + `--jsonl`が動作する
- [ ] 複数ファイル + `-o`（JSONL結合）が動作する
- [ ] 複数ファイル + `-d`のみ（個別JSON）が動作する
- [ ] 複数ファイル + `-d` + `--jsonl`（自動命名JSONL）が動作する
- [ ] 複数ファイル + `--jsonl`のみ（カレントに自動命名JSONL）が動作する
- [ ] JSONL自動命名が正しい形式（`{first}_3files.jsonl`）で生成される
- [ ] `-o`指定の警告メッセージが削除されている（複数ファイル時は常にJSONL）
- [ ] README.mdのCLI使用例が更新されている
- [ ] `cargo fmt`実行済み
- [ ] `cargo clippy`で警告なし
- [ ] 全テストが成功する

## 使用例

### 単一ファイル
```bash
# 整形JSON（デフォルト）
nksf-parser-cli preset.nksf

# コンパクトJSON（1行）
nksf-parser-cli preset.nksf --jsonl

# ファイル出力
nksf-parser-cli preset.nksf -o output.json       # 整形JSON
nksf-parser-cli preset.nksf -o output.jsonl --jsonl  # コンパクトJSON
```

### 複数ファイル
```bash
# 個別JSON出力（カレント）
nksf-parser-cli preset1.nksf preset2.nksf preset3.nksf

# 個別JSON出力（ディレクトリ指定）
nksf-parser-cli -d ./output preset1.nksf preset2.nksf

# JSONL結合出力（ファイル指定）
nksf-parser-cli -o combined.jsonl preset1.nksf preset2.nksf preset3.nksf

# JSONL自動命名（カレント）
nksf-parser-cli preset1.nksf preset2.nksf preset3.nksf --jsonl
# → ./preset1_3files.jsonl

# JSONL自動命名（ディレクトリ指定）
nksf-parser-cli -d ./output preset1.nksf preset2.nksf preset3.nksf --jsonl
# → ./output/preset1_3files.jsonl
```

## 注意事項

### 破壊的変更
- `-c, --compact`オプションの削除は破壊的変更
- 既存ユーザーへの影響を考慮

### JSONLの利点
- ストリーミング処理が可能
- 大量ファイルの結合に最適
- 行単位での処理が容易

### エラーハンドリング
- 既存のエラーメッセージスタイルを維持
- JSONL出力時もエラーはstderrに出力

---

**Status**: Not Started
**Priority**: Medium
**Estimated Time**: 1-2時間
**Dependencies**: タスク019完了
