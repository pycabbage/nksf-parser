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
/// * `MessagePackError` - `MessagePackデシリアライズエラー`
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
/// * `MessagePackError` - `MessagePackデシリアライズエラー`
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

/// NISIメタデータの必須フィールドを検証
///
/// 必須フィールドが空でないことを確認する
fn validate_nisi_metadata(metadata: &NisiMetadata) -> Result<()> {
    if metadata.name.is_empty() {
        return Err(ParseError::InvalidNiks);
    }
    if metadata.vendor.is_empty() {
        return Err(ParseError::InvalidNiks);
    }
    if metadata.device_type.is_empty() {
        return Err(ParseError::InvalidNiks);
    }
    Ok(())
}

/// NICAパラメータのID重複を検証
///
/// 同じパラメータIDが複数回使用されていないことを確認する
fn validate_nica_parameters(parameters: &NicaData) -> Result<()> {
    use std::collections::HashSet;

    for array in &parameters.ni8 {
        if let Some(params) = array.as_array() {
            let mut ids = HashSet::new();
            for param in params {
                if let Some(id) = param.get("id").and_then(serde_json::Value::as_u64)
                    && !ids.insert(id)
                {
                    return Err(ParseError::InvalidNiks);
                }
            }
        }
    }
    Ok(())
}

/// 内部ヘルパー関数: Readerから.nksfファイルを解析
fn parse_nksf_from_reader<R: Read + Seek>(reader: R) -> Result<NksfFile> {
    let mut riff_reader = RiffReader::new(reader)?;

    let mut metadata: Option<NisiMetadata> = None;
    let mut parameters: Option<NicaData> = None;
    let mut plugin_id: Option<PlidData> = None;
    let mut plugin_chunk: Option<PchkData> = None;

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
                // NKSFファイルは4つのチャンク（NISI, NICA, PLID, PCHK）のみを含む
                // 他のチャンクが見つかった場合は不正なファイル
                return Err(ParseError::UnknownChunk(chunk_id.to_string()));
            }
        }
    }

    riff_reader.verify_complete()?;

    // 4つの必須チャンクがすべて存在することを確認
    let metadata = metadata.ok_or(ParseError::InvalidNiks)?;
    let parameters = parameters.ok_or(ParseError::InvalidNiks)?;
    let plugin_id = plugin_id.ok_or(ParseError::InvalidNiks)?;
    let plugin_chunk = plugin_chunk.ok_or(ParseError::InvalidNiks)?;

    // フィールドレベルのバリデーション
    validate_nisi_metadata(&metadata)?;
    validate_nica_parameters(&parameters)?;

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

    #[test]
    fn test_validate_nisi_empty_name() {
        let metadata = NisiMetadata {
            ni_internal: serde_json::Value::String("BRIB".to_string()),
            author: "Test".to_string(),
            bankchain: vec![],
            characters: vec![],
            comment: "Test".to_string(),
            device_type: "INST".to_string(),
            modes: vec![],
            name: String::new(), // 空の名前
            types: vec![],
            uuid: "test".to_string(),
            vendor: "Test".to_string(),
        };

        let result = validate_nisi_metadata(&metadata);
        assert!(matches!(result, Err(ParseError::InvalidNiks)));
    }

    #[test]
    fn test_validate_nisi_empty_vendor() {
        let metadata = NisiMetadata {
            ni_internal: serde_json::Value::String("BRIB".to_string()),
            author: "Test".to_string(),
            bankchain: vec![],
            characters: vec![],
            comment: "Test".to_string(),
            device_type: "INST".to_string(),
            modes: vec![],
            name: "Test".to_string(),
            types: vec![],
            uuid: "test".to_string(),
            vendor: String::new(), // 空のベンダー
        };

        let result = validate_nisi_metadata(&metadata);
        assert!(matches!(result, Err(ParseError::InvalidNiks)));
    }

    #[test]
    fn test_validate_nica_duplicate_id() {
        // パラメータIDが重複しているNicaDataを作成
        let nica_data = NicaData {
            ni8: vec![serde_json::json!([
                {"id": 0, "name": "Param 0", "autoname": true, "vflag": false},
                {"id": 0, "name": "Param 0 Duplicate", "autoname": true, "vflag": false}, // 重複ID
            ])],
        };

        let result = validate_nica_parameters(&nica_data);
        assert!(matches!(result, Err(ParseError::InvalidNiks)));
    }
}
