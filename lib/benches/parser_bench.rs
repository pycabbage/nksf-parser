use criterion::{Criterion, criterion_group, criterion_main};
use nksf_parser::parse_nksf_from_bytes;
use std::fs;
use std::hint::black_box;
use std::path::PathBuf;

/// Abandoned.nksfの解析ベンチマーク
fn bench_parse_abandoned(c: &mut Criterion) {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("massive_x_factory_library_tests")
        .join("fixture")
        .join("Abandoned.nksf");

    let data = fs::read(&path).expect("Failed to read fixture file");

    c.bench_function("parse_abandoned", |b| {
        b.iter(|| parse_nksf_from_bytes(black_box(&data)));
    });
}

/// 複数のプリセットファイルの解析ベンチマーク
fn bench_parse_multiple_presets(c: &mut Criterion) {
    let fixture_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("massive_x_factory_library_tests")
        .join("fixture");

    // 最初の10個のファイルを読み込み
    let mut files = Vec::new();
    for entry in fs::read_dir(&fixture_dir).unwrap().take(10) {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("nksf") {
            let data = fs::read(&path).unwrap();
            files.push(data);
        }
    }

    c.bench_function("parse_10_presets", |b| {
        b.iter(|| {
            for data in &files {
                let _ = parse_nksf_from_bytes(black_box(data));
            }
        });
    });
}

criterion_group!(benches, bench_parse_abandoned, bench_parse_multiple_presets);
criterion_main!(benches);
