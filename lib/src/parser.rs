use crate::error::{ParseError, Result};
use crate::nica_parser::parse_nica_chunk;
use crate::nisi_parser::parse_nisi_chunk;
use crate::pchk_parser::parse_pchk_chunk;
use crate::plid_parser::parse_plid_chunk;
use crate::riff_reader::RiffReader;
use crate::types::{NicaData, NisiMetadata, NksfFile, PchkData, PlidData};
use std::fs::File;
use std::io::{BufReader, Cursor, Read, Seek};
use std::path::Path;

/// .nksfファイルをパスから解析
///
/// # Arguments
/// * `path` - .nksfファイルのパス
///
/// # Returns
/// * `Result<NksfFile>` - 解析されたファイルデータ
///
/// # Errors
/// * `IoError` - ファイルが存在しない、または読み取れない
/// * `InvalidRiff` - 不正なRIFFフォーマット
/// * `InvalidNiks` - 不正なNIKSフォーマット、または必須チャンクが欠けている
/// * `MessagePackError` - MessagePackデシリアライズエラー
/// * `IncompleteParse` - 未解析のバイトが残っている
///
/// # Examples
///
/// ```no_run
/// use nksf_parser::parse_nksf;
/// use std::path::Path;
///
/// let path = Path::new("preset.nksf");
/// let nksf = parse_nksf(&path)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn parse_nksf(path: &Path) -> Result<NksfFile> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    parse_nksf_from_reader(reader)
}

/// .nksfファイルをバイト配列から解析
///
/// # Arguments
/// * `data` - .nksfファイルのバイトデータ
///
/// # Returns
/// * `Result<NksfFile>` - 解析されたファイルデータ
///
/// # Errors
/// * `InvalidRiff` - 不正なRIFFフォーマット
/// * `InvalidNiks` - 不正なNIKSフォーマット、または必須チャンクが欠けている
/// * `MessagePackError` - MessagePackデシリアライズエラー
/// * `IncompleteParse` - 未解析のバイトが残っている
///
/// # Examples
///
/// ```no_run
/// use nksf_parser::parse_nksf_from_bytes;
///
/// let data = std::fs::read("preset.nksf")?;
/// let nksf = parse_nksf_from_bytes(&data)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn parse_nksf_from_bytes(data: &[u8]) -> Result<NksfFile> {
    let cursor = Cursor::new(data);
    parse_nksf_from_reader(cursor)
}

/// 内部ヘルパー関数: Readerから.nksfファイルを解析
fn parse_nksf_from_reader<R: Read + Seek>(reader: R) -> Result<NksfFile> {
    let mut riff_reader = RiffReader::new(reader)?;

    let mut metadata: Option<NisiMetadata> = None;
    let mut parameters: Option<NicaData> = None;
    let mut plugin_id: Option<PlidData> = None;
    let mut plugin_chunk: Option<PchkData> = None;

    // 全チャンクを処理
    while let Some(chunk) = riff_reader.next_chunk()? {
        let chunk_id = std::str::from_utf8(&chunk.id).unwrap_or("????");
        let data = riff_reader.read_chunk_data(&chunk)?;

        match chunk_id {
            "NISI" => {
                metadata = Some(parse_nisi_chunk(&data)?);
            }
            "NICA" => {
                parameters = Some(parse_nica_chunk(&data)?);
            }
            "PLID" => {
                plugin_id = Some(parse_plid_chunk(&data)?);
            }
            "PCHK" => {
                plugin_chunk = Some(parse_pchk_chunk(&data)?);
            }
            _ => {
                // 未知のチャンクが見つかった場合はエラー
                return Err(ParseError::UnknownChunk(chunk_id.to_string()));
            }
        }
    }

    // 全バイトが読み取られたことを検証
    riff_reader.verify_complete()?;

    // 必須チャンクの確認
    let metadata = metadata.ok_or(ParseError::InvalidNiks)?;
    let parameters = parameters.ok_or(ParseError::InvalidNiks)?;
    let plugin_id = plugin_id.ok_or(ParseError::InvalidNiks)?;
    let plugin_chunk = plugin_chunk.ok_or(ParseError::InvalidNiks)?;

    Ok(NksfFile {
        metadata,
        parameters,
        plugin_id,
        plugin_chunk,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_nksf_from_bytes_invalid_riff() {
        let data = b"INVALID DATA";
        let result = parse_nksf_from_bytes(data);
        assert!(matches!(result, Err(ParseError::InvalidRiff)));
    }

    #[test]
    fn test_parse_nksf_from_bytes_missing_chunks() {
        // RIFFヘッダーのみ、チャンクなし
        let data = b"RIFF\x04\x00\x00\x00NIKS";
        let result = parse_nksf_from_bytes(data);
        // 必須チャンクが欠けている
        assert!(matches!(result, Err(ParseError::InvalidNiks)));
    }
}
