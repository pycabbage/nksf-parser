use crate::error::{ParseError, Result};

/// `バージョン付きMessagePackデータを解析`
///
/// NISIチャンク、NICAチャンク、PLIDチャンクで共通の処理
///
/// # Arguments
/// * `data` - チャンクデータ（バージョン情報を含む）
///
/// # Returns
/// * `Result<T>` - 解析されたデータ
///
/// # Errors
/// * `InvalidNiks` - データサイズが不正、またはサポートされていないバージョン
/// * `MessagePackError` - `MessagePackデシリアライズエラー`
pub fn parse_versioned_msgpack<T: serde::de::DeserializeOwned>(data: &[u8]) -> Result<T> {
    if data.len() < 4 {
        return Err(ParseError::InvalidNiks);
    }

    let version = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);

    // 将来的なフォーマット変更への対応として、バージョン1のみをサポート
    // 他のバージョンが見つかった場合は明示的にエラーを返す
    if version != 1 {
        return Err(ParseError::InvalidNiks);
    }

    let result: T = rmp_serde::from_slice(&data[4..])?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_versioned_msgpack_too_short() {
        // バージョン情報（4バイト）に満たないデータを渡した場合、
        // エラーを返すことを確認
        let data = [0x01, 0x02, 0x03];
        let result = parse_versioned_msgpack::<serde_json::Value>(&data);
        assert!(matches!(result, Err(ParseError::InvalidNiks)));
    }

    #[test]
    fn test_parse_versioned_msgpack_unsupported_version() {
        // バージョン2など、サポートされていないバージョンの場合、
        // エラーを返すことを確認
        let data = [0x02, 0x00, 0x00, 0x00];
        let result = parse_versioned_msgpack::<serde_json::Value>(&data);
        assert!(matches!(result, Err(ParseError::InvalidNiks)));
    }

    #[test]
    fn test_parse_versioned_msgpack_invalid_data() {
        // バージョン1の後に不正なMessagePackデータを配置
        // 0xc1はMessagePackの予約済みバイトで、有効なデータではない
        // パーサーがエラーを適切に返すことを確認
        let mut data = vec![0x01, 0x00, 0x00, 0x00]; // バージョン1
        data.extend_from_slice(&[0xc1]); // 予約済みバイト
        let result = parse_versioned_msgpack::<serde_json::Value>(&data);
        assert!(result.is_err());
    }
}
