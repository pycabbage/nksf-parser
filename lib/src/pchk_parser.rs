use crate::error::{ParseError, Result};
use crate::types::{PchkData, PchkHeader};
use flate2::read::ZlibDecoder;
use std::io::Read;

/// PCHKヘッダーサイズ（バイト）
const PCHK_HEADER_SIZE: usize = 20;

/// 展開後データの最大サイズ（50MB）
/// Zlib解凍爆弾攻撃を防ぐため
const MAX_DECOMPRESSED_SIZE: usize = 50 * 1024 * 1024;

/// MessagePack値の最大数（無限ループ防止）
const MAX_MSGPACK_VALUES: usize = 100_000;

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
/// * `MessagePackError` - `MessagePackデシリアライズエラー`
/// * `IncompleteParse` - 展開後のデータに未解析バイトが残っている
pub fn parse_pchk_chunk(data: &[u8]) -> Result<PchkData> {
    if data.len() < PCHK_HEADER_SIZE {
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

    // zlibで圧縮されたデータを展開する
    // 攻撃者が数KBの圧縮データから数GBの展開データを生成する
    // "Zip Bomb"攻撃を防ぐため、展開サイズに上限を設ける
    let compressed_data = &data[PCHK_HEADER_SIZE..];
    let decoder = ZlibDecoder::new(compressed_data);
    let mut decompressed = Vec::new();

    // 最大50MBまでしか展開しない（通常のプリセットは数百KB程度）
    decoder
        .take(MAX_DECOMPRESSED_SIZE as u64)
        .read_to_end(&mut decompressed)?;

    // 上限に達した場合は不正なファイルと判断
    if decompressed.len() >= MAX_DECOMPRESSED_SIZE {
        return Err(ParseError::InvalidNiks);
    }

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

        // 不正なデータによる無限ループを防ぐため、値の数に上限を設ける
        // 通常のプリセットは約268個の値を持つため、10万個は十分な余裕がある
        if values.len() > MAX_MSGPACK_VALUES {
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
        // サポートされていないバージョン（2）を含むPCHKチャンクで
        // エラーが返されることを確認
        let mut data = vec![0x02, 0x00, 0x00, 0x00];
        data.extend_from_slice(&[0; 16]);
        let result = parse_pchk_chunk(&data);
        assert!(matches!(result, Err(ParseError::InvalidNiks)));
    }

    #[test]
    fn test_pchk_invalid_zlib() {
        // zlibマジックナンバー（0x78 0x9c）ではない不正な圧縮データで
        // エラーが返されることを確認
        let mut data = vec![0x01, 0x00, 0x00, 0x00];
        data.extend_from_slice(&[0x02, 0x00, 0x00, 0x00]);
        data.extend_from_slice(&[0x02, 0x00, 0x00, 0x00]);
        data.extend_from_slice(&[0x10, 0x00, 0x00, 0x00]);
        data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
        data.extend_from_slice(&[0xFF, 0xFF, 0xFF]);
        let result = parse_pchk_chunk(&data);
        assert!(result.is_err());
    }
}
