mod massive_x_factory_library_tests;

use nksf_parser::{ParseError, parse_nksf_from_bytes};
use std::path::PathBuf;

// =====================================
// エラーケースのテスト
// =====================================

#[test]
fn test_invalid_riff_file() {
    let invalid_data = b"INVALID DATA";
    let result = parse_nksf_from_bytes(invalid_data);
    assert!(matches!(result, Err(ParseError::InvalidRiff)));
}

#[test]
fn test_incomplete_file() {
    // RIFFヘッダーのみのファイル
    let incomplete_data = b"RIFF\x04\x00\x00\x00NIKS";
    let result = parse_nksf_from_bytes(incomplete_data);
    assert!(matches!(result, Err(ParseError::InvalidNiks)));
}

#[test]
fn test_nonexistent_file() {
    let path = PathBuf::from("/nonexistent/file.nksf");
    let result = nksf_parser::parse_nksf(&path);
    assert!(matches!(result, Err(ParseError::IoError(_))));
}

#[test]
fn test_invalid_niks_format() {
    // RIFFヘッダーは正しいが、フォーマット識別子が違う
    let invalid_data = b"RIFF\x10\x00\x00\x00INVALID";
    let result = parse_nksf_from_bytes(invalid_data);
    assert!(matches!(result, Err(ParseError::InvalidNiks)));
}

#[test]
fn test_unknown_chunk() {
    // 未知のチャンクを含むファイル
    let mut data = Vec::new();
    data.extend_from_slice(b"RIFF");
    data.extend_from_slice(&20u32.to_le_bytes());
    data.extend_from_slice(b"NIKS");
    data.extend_from_slice(b"UNKN"); // 未知のチャンクID
    data.extend_from_slice(&4u32.to_le_bytes());
    data.extend_from_slice(b"TEST");

    let result = parse_nksf_from_bytes(&data);
    assert!(matches!(result, Err(ParseError::UnknownChunk(_))));
}

// =====================================
// パフォーマンステスト（オプション）
// =====================================

#[test]
#[ignore] // 通常のテスト実行では除外
fn test_parse_performance() {
    use std::time::Instant;

    let fixture_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("massive_x_factory_library_tests")
        .join("fixture");

    let mut total_time = std::time::Duration::ZERO;
    let mut file_count = 0;

    for entry in std::fs::read_dir(&fixture_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("nksf") {
            let start = Instant::now();
            let _ = nksf_parser::parse_nksf(&path).expect("Parse failed");
            total_time += start.elapsed();
            file_count += 1;
        }
    }

    let avg_time = total_time / file_count;
    println!(
        "Parsed {} files in {:?} (avg: {:?} per file)",
        file_count, total_time, avg_time
    );

    // 1ファイルあたり10ms以内の目標
    assert!(
        avg_time.as_millis() < 10,
        "Parse time too slow: {:?}",
        avg_time
    );
}
