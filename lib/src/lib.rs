// モジュール宣言
mod error;
mod types;
mod riff_reader;
mod nisi_parser;
mod nica_parser;

// パブリックAPIのエクスポート
pub use error::{ParseError, Result};
pub use types::{
    NksfFile,
    NisiMetadata,
    NiInternal,
    NicaData,
    Parameter,
    UnknownChunk,
};
