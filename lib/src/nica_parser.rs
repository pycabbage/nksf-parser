use crate::error::Result;
use crate::msgpack_utils::parse_versioned_msgpack;
use crate::types::NicaData;

/// NICAチャンクデータを解析
///
/// # Arguments
/// * `data` - チャンクデータ（バージョン情報を含む）
///
/// # Returns
/// * `Result<NicaData>` - 解析されたパラメータデータ
///
/// # Errors
/// * `InvalidNiks` - データサイズが不正、またはサポートされていないバージョン
/// * `MessagePackError` - `MessagePackデシリアライズエラー`
pub fn parse_nica_chunk(data: &[u8]) -> Result<NicaData> {
    parse_versioned_msgpack(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::ParseError;

    #[test]
    fn test_parse_nica_chunk_too_short() {
        let data = [0x01, 0x02, 0x03]; // 3バイトのみ
        let result = parse_nica_chunk(&data);
        assert!(matches!(result, Err(ParseError::InvalidNiks)));
    }

    #[test]
    fn test_parse_nica_chunk_unsupported_version() {
        let data = [0x02, 0x00, 0x00, 0x00]; // バージョン2
        let result = parse_nica_chunk(&data);
        assert!(matches!(result, Err(ParseError::InvalidNiks)));
    }

    #[test]
    fn test_parse_nica_chunk_invalid_messagepack() {
        let mut data = vec![0x01, 0x00, 0x00, 0x00]; // バージョン1
        data.extend_from_slice(&[0xFF, 0xFF, 0xFF]); // 不正なMessagePackデータ
        let result = parse_nica_chunk(&data);
        assert!(result.is_err());
    }
}
