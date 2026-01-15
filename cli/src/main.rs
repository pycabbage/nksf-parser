use clap::Parser;
use nksf_parser::ParseError;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::process;

/// .nksfファイルをJSON形式で出力するCLIツール
#[derive(Parser)]
#[command(
    name = "nksf-parser-cli",
    version,
    about = "Parse .nksf (Massive X preset) files and output as JSON"
)]
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

    /// Output compact JSON (no pretty printing)
    #[arg(short, long)]
    compact: bool,

    /// Overwrite existing files without asking
    #[arg(long)]
    overwrite: bool,
}

fn main() {
    let args = Args::parse();

    let result = if args.input.len() == 1 {
        process_single_file(
            &args.input[0],
            args.output.as_deref(),
            args.dir.as_deref(),
            args.compact,
            args.overwrite,
        )
    } else {
        process_multiple_files(
            &args.input,
            args.dir.as_deref(),
            args.compact,
            args.overwrite,
            args.output.is_some(),
        )
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

/// 単一ファイルを処理
fn process_single_file(
    input: &Path,
    output: Option<&Path>,
    dir: Option<&Path>,
    compact: bool,
    overwrite: bool,
) -> Result<(), Box<dyn Error>> {
    let nksf = parse_with_error_hints(input)?;
    let json_output = serialize_json(&nksf, compact)?;

    match (output, dir) {
        (None, None) => {
            // stdoutへ出力
            println!("{json_output}");
        }
        (Some(output_file), None) => {
            // ファイルへ出力
            write_to_file(output_file, &json_output, overwrite)?;
        }
        (None, Some(output_dir)) => {
            // ディレクトリ指定
            verify_directory_exists(output_dir);
            let output_file = generate_output_path(input, output_dir);
            write_to_file(&output_file, &json_output, overwrite)?;
        }
        (Some(output_file), Some(output_dir)) => {
            // ディレクトリ + ファイル名
            verify_directory_exists(output_dir);
            let output_path = output_dir.join(output_file);
            write_to_file(&output_path, &json_output, overwrite)?;
        }
    }

    Ok(())
}

/// 複数ファイルを処理
fn process_multiple_files(
    inputs: &[PathBuf],
    dir: Option<&Path>,
    compact: bool,
    overwrite: bool,
    output_specified: bool,
) -> Result<(), Box<dyn Error>> {
    if output_specified {
        eprintln!("Warning: -o option is ignored when multiple input files are specified");
    }

    let output_dir = dir.unwrap_or_else(|| Path::new("."));
    verify_directory_exists(output_dir);

    for input in inputs {
        let nksf = parse_with_error_hints(input)?;
        let json_output = serialize_json(&nksf, compact)?;

        let output_file = generate_output_path(input, output_dir);
        write_to_file(&output_file, &json_output, overwrite)?;

        eprintln!(
            "✓ Processed: {} -> {}",
            input.display(),
            output_file.display()
        );
    }

    Ok(())
}

/// ファイルを解析し、エラー時にヒントを表示
fn parse_with_error_hints(input: &Path) -> Result<nksf_parser::NksfFile, Box<dyn Error>> {
    match nksf_parser::parse_nksf(input) {
        Ok(nksf) => Ok(nksf),
        Err(e) => {
            eprintln!("Error: Failed to parse '{}'", input.display());
            eprintln!("Reason: {e}");

            match &e {
                ParseError::IoError(_) => {
                    eprintln!("Hint: Check if the file exists and you have read permission.");
                }
                ParseError::InvalidRiff => {
                    eprintln!("Hint: The file is not a valid RIFF file.");
                }
                ParseError::InvalidNiks => {
                    eprintln!("Hint: The file is not a valid NKSF file or contains invalid data.");
                }
                ParseError::UnknownChunk(chunk_id) => {
                    eprintln!("Hint: The file contains an unknown chunk type: {chunk_id}");
                }
                ParseError::IncompleteParse(remaining, offset) => {
                    eprintln!(
                        "Hint: {remaining} bytes at offset {offset} were not parsed. The file may be corrupted."
                    );
                }
                ParseError::MessagePackError(_) => {}
            }

            Err(Box::new(e))
        }
    }
}

/// JSON文字列にシリアライズ
fn serialize_json(nksf: &nksf_parser::NksfFile, compact: bool) -> Result<String, Box<dyn Error>> {
    let json = if compact {
        serde_json::to_string(nksf)?
    } else {
        serde_json::to_string_pretty(nksf)?
    };
    Ok(json)
}

/// 入力ファイル名から出力ファイルパスを生成
fn generate_output_path(input: &Path, output_dir: &Path) -> PathBuf {
    let file_stem = input.file_stem().unwrap_or_default();
    output_dir.join(format!("{}.json", file_stem.to_string_lossy()))
}

/// ファイルへ書き込み（上書きチェック付き）
fn write_to_file(path: &Path, content: &str, overwrite: bool) -> Result<(), Box<dyn Error>> {
    if path.exists() && !overwrite {
        eprintln!("Error: Output file '{}' already exists", path.display());
        eprintln!("Hint: Use --overwrite to overwrite existing files");
        process::exit(1);
    }

    std::fs::write(path, content)?;
    Ok(())
}

/// ディレクトリの存在を確認
fn verify_directory_exists(dir: &Path) {
    if !dir.exists() {
        eprintln!("Error: Output directory '{}' does not exist", dir.display());
        eprintln!("Hint: Create the directory first or check the path");
        process::exit(1);
    }

    if !dir.is_dir() {
        eprintln!("Error: '{}' is not a directory", dir.display());
        process::exit(1);
    }
}
