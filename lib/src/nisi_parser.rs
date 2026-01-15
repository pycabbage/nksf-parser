use crate::error::Result;
use crate::msgpack_utils::parse_versioned_msgpack;
use crate::types::NisiMetadata;

/// NISIチャンクデータを解析
///
/// # Arguments
/// * `data` - チャンクデータ（バージョン情報を含む）
///
/// # Returns
/// * `Result<NisiMetadata>` - 解析されたメタデータ
///
/// # Errors
/// * `InvalidNiks` - データサイズが不正、またはサポートされていないバージョン
/// * `MessagePackError` - `MessagePackデシリアライズエラー`
pub fn parse_nisi_chunk(data: &[u8]) -> Result<NisiMetadata> {
    parse_versioned_msgpack(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::ParseError;

    #[test]
    fn test_parse_nisi_chunk_too_short() {
        let data = [0x01, 0x02, 0x03]; // 3バイトのみ
        let result = parse_nisi_chunk(&data);
        assert!(matches!(result, Err(ParseError::InvalidNiks)));
    }

    #[test]
    fn test_parse_nisi_chunk_unsupported_version() {
        let data = [0x02, 0x00, 0x00, 0x00]; // バージョン2
        let result = parse_nisi_chunk(&data);
        assert!(matches!(result, Err(ParseError::InvalidNiks)));
    }

    #[test]
    fn test_parse_nisi_chunk_invalid_messagepack() {
        let mut data = vec![0x01, 0x00, 0x00, 0x00]; // バージョン1
        data.extend_from_slice(&[0xFF, 0xFF, 0xFF]); // 不正なMessagePackデータ
        let result = parse_nisi_chunk(&data);
        assert!(result.is_err());
    }
}
