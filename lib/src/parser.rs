use crate::error::{ParseError, Result};
use crate::riff_reader::RiffReader;
use crate::nisi_parser::parse_nisi_chunk;
use crate::nica_parser::parse_nica_chunk;
use crate::types::{NksfFile, NisiMetadata, NicaData, UnknownChunk};
use std::path::Path;
use std::fs::File;
use std::io::{BufReader, Cursor, Read, Seek};

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
    let mut unknown_chunks: Vec<UnknownChunk> = Vec::new();

    // 全チャンクを処理
    while let Some(chunk) = riff_reader.next_chunk()? {
        let chunk_id = std::str::from_utf8(&chunk.id)
            .unwrap_or("????");
        let data = riff_reader.read_chunk_data(&chunk)?;

        match chunk_id {
            "NISI" => {
                metadata = Some(parse_nisi_chunk(&data)?);
            }
            "NICA" => {
                parameters = Some(parse_nica_chunk(&data)?);
            }
            _ => {
                // 未知のチャンクを保存（完全なバイト解析のため）
                unknown_chunks.push(UnknownChunk {
                    id: chunk_id.to_string(),
                    version: None, // 必要に応じてバージョンを抽出
                    data,
                });
            }
        }
    }

    // 全バイトが読み取られたことを検証
    riff_reader.verify_complete()?;

    // 必須チャンクの確認
    let metadata = metadata.ok_or(ParseError::InvalidNiks)?;
    let parameters = parameters.ok_or(ParseError::InvalidNiks)?;

    Ok(NksfFile {
        metadata,
        parameters,
        unknown_chunks,
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
