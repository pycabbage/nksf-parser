use clap::Parser;
use nksf_parser::ParseError;
use std::path::PathBuf;
use std::process;

/// .nksfファイルをJSON形式で出力するCLIツール
#[derive(Parser)]
#[command(name = "nksf-parser-cli")]
#[command(about = "Parse .nksf (Massive X preset) files and output as JSON")]
struct Args {
    /// Input .nksf file path
    input: PathBuf,

    /// Output compact JSON (no pretty printing)
    #[arg(short, long)]
    compact: bool,
}

fn main() {
    let args = Args::parse();

    // ファイルを解析
    let result = match nksf_parser::parse_nksf(&args.input) {
        Ok(nksf) => nksf,
        Err(e) => {
            eprintln!("Error: Failed to parse '{}'", args.input.display());
            eprintln!("Reason: {e}");

            // エラーの種類に応じた追加情報を提供
            match e {
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

            process::exit(1);
        }
    };

    // JSON出力
    let json_output = if args.compact {
        match serde_json::to_string(&result) {
            Ok(json) => json,
            Err(e) => {
                eprintln!("Error serializing to JSON: {e}");
                process::exit(1);
            }
        }
    } else {
        match serde_json::to_string_pretty(&result) {
            Ok(json) => json,
            Err(e) => {
                eprintln!("Error serializing to JSON: {e}");
                process::exit(1);
            }
        }
    };

    println!("{json_output}");
}
