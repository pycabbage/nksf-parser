//! # nksf-parser
//!
//! .nksf (Massive Xのプリセットファイル) のパーサーライブラリ
//!
//! このライブラリは、Native Instruments Massive Xのプリセットファイル (.nksf) を
//! 解析し、Rust構造体として扱えるようにします。
//!
//! ## 機能
//!
//! - RIFFフォーマットの解析
//! - MessagePackデータのデシリアライズ
//! - メタデータとパラメータ情報の抽出
//! - 完全なバイト解析（データの欠損なし）
//!
//! ## 使用例
//!
//! ```no_run
//! use nksf_parser::parse_nksf;
//! use std::path::Path;
//!
//! let path = Path::new("preset.nksf");
//! let result = parse_nksf(&path).unwrap();
//!
//! println!("Preset name: {}", result.metadata.name);
//! println!("Author: {}", result.metadata.author);
//! ```

// モジュール宣言
mod error;
mod msgpack_utils;
mod nica_parser;
mod nisi_parser;
mod parser;
mod pchk_parser;
mod plid_parser;
mod riff_reader;
mod types;

// パブリックAPIのエクスポート
pub use error::{ParseError, Result};
pub use parser::{parse_nksf, parse_nksf_from_bytes};
pub use types::{
    NiInternal, NicaData, NisiMetadata, NksfFile, Parameter, PchkData, PchkHeader, PlidData,
};
