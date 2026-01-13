use crate::error::{ParseError, Result};
use crate::types::{PchkData, PchkHeader};
use flate2::read::ZlibDecoder;
use std::io::Read;

/// PCHKチャンクデータを解析
///
/// # Arguments
/// * `data` - チャンクデータ（ヘッダー + zlib圧縮データ）
///
/// # Returns
/// * `Result<PchkData>` - 解析されたプラグインチャンク
///
/// # Errors
/// * `InvalidNiks` - データサイズが不正、またはサポートされていないバージョン
/// * `IoError` - zlib展開エラー
/// * `MessagePackError` - MessagePackデシリアライズエラー
/// * `IncompleteParse` - 展開後のデータに未解析バイトが残っている
pub fn parse_pchk_chunk(data: &[u8]) -> Result<PchkData> {
    if data.len() < 20 {
        return Err(ParseError::InvalidNiks);
    }

    // ヘッダー解析
    let header = PchkHeader {
        version: u32::from_le_bytes([data[0], data[1], data[2], data[3]]),
        field1: u32::from_le_bytes([data[4], data[5], data[6], data[7]]),
        field2: u32::from_le_bytes([data[8], data[9], data[10], data[11]]),
        compressed_size: u32::from_le_bytes([data[12], data[13], data[14], data[15]]),
        field3: u32::from_le_bytes([data[16], data[17], data[18], data[19]]),
    };

    if header.version != 1 {
        return Err(ParseError::InvalidNiks);
    }

    // zlib展開
    let compressed_data = &data[20..];
    let mut decoder = ZlibDecoder::new(compressed_data);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)?;

    // MessagePackストリームから全値を読み取る
    let mut cursor = std::io::Cursor::new(&decompressed[..]);
    let mut values = Vec::new();

    loop {
        let pos = cursor.position();
        if pos >= decompressed.len() as u64 {
            break;
        }

        let value: serde_json::Value = rmp_serde::from_read(&mut cursor)?;
        values.push(value);

        // 安全装置（無限ループ防止）
        if values.len() > 100000 {
            return Err(ParseError::InvalidNiks);
        }
    }

    // 全バイトが消費されたことを確認
    if cursor.position() != decompressed.len() as u64 {
        return Err(ParseError::IncompleteParse(
            (decompressed.len() as u64 - cursor.position()) as usize,
            cursor.position() as usize,
        ));
    }

    Ok(PchkData { header, values })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pchk_too_short() {
        let data = [0x01, 0x02];
        let result = parse_pchk_chunk(&data);
        assert!(matches!(result, Err(ParseError::InvalidNiks)));
    }

    #[test]
    fn test_pchk_unsupported_version() {
        let mut data = vec![0x02, 0x00, 0x00, 0x00]; // バージョン2
        data.extend_from_slice(&[0; 16]); // 残りのヘッダー
        let result = parse_pchk_chunk(&data);
        assert!(matches!(result, Err(ParseError::InvalidNiks)));
    }

    #[test]
    fn test_pchk_invalid_zlib() {
        let mut data = vec![0x01, 0x00, 0x00, 0x00]; // バージョン1
        data.extend_from_slice(&[0x02, 0x00, 0x00, 0x00]); // field1
        data.extend_from_slice(&[0x02, 0x00, 0x00, 0x00]); // field2
        data.extend_from_slice(&[0x10, 0x00, 0x00, 0x00]); // compressed_size
        data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // field3
        data.extend_from_slice(&[0xFF, 0xFF, 0xFF]); // 不正なzlibデータ
        let result = parse_pchk_chunk(&data);
        assert!(result.is_err());
    }
}
