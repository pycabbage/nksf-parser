use thiserror::Error;

/// パーサーエラー型
#[derive(Error, Debug)]
pub enum ParseError {
    /// 不正なRIFFフォーマット
    #[error("Invalid RIFF format")]
    InvalidRiff,

    /// 不正なNIKSフォーマット
    #[error("Invalid NIKS format")]
    InvalidNiks,

    /// `MessagePackデシリアライズエラー`
    #[error("MessagePack deserialization error: {0}")]
    MessagePackError(#[from] rmp_serde::decode::Error),

    /// I/Oエラー
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// 未知のチャンク
    #[error("Unknown chunk: {0}")]
    UnknownChunk(String),

    /// 未解析のバイトが残っている
    #[error("Incomplete parse: {0} bytes remaining at offset {1}")]
    IncompleteParse(usize, usize),
}

/// パーサーの結果型
pub type Result<T> = std::result::Result<T, ParseError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = ParseError::InvalidRiff;
        assert_eq!(err.to_string(), "Invalid RIFF format");
    }

    #[test]
    fn test_invalid_niks_error() {
        let err = ParseError::InvalidNiks;
        assert_eq!(err.to_string(), "Invalid NIKS format");
    }

    #[test]
    fn test_unknown_chunk_error() {
        let err = ParseError::UnknownChunk("TEST".to_string());
        assert_eq!(err.to_string(), "Unknown chunk: TEST");
    }

    #[test]
    fn test_incomplete_parse_error() {
        let err = ParseError::IncompleteParse(100, 500);
        assert_eq!(
            err.to_string(),
            "Incomplete parse: 100 bytes remaining at offset 500"
        );
    }
}
