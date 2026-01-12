use crate::error::{ParseError, Result};
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
/// * `MessagePackError` - MessagePackデシリアライズエラー
pub fn parse_nisi_chunk(data: &[u8]) -> Result<NisiMetadata> {
    // バージョンの読み取り（最初の4バイト、リトルエンディアン）
    if data.len() < 4 {
        return Err(ParseError::InvalidNiks);
    }

    let version = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);

    // バージョン1のみサポート
    if version != 1 {
        return Err(ParseError::InvalidNiks);
    }

    // MessagePackデータのデシリアライズ
    let metadata: NisiMetadata = rmp_serde::from_slice(&data[4..])?;

    Ok(metadata)
}

#[cfg(test)]
mod tests {
    use super::*;

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
