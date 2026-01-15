use crate::error::{ParseError, Result};
use std::io::{Read, Seek, SeekFrom};

/// チャンクサイズの最大値（100MB）
/// メモリ枯渇攻撃を防ぐため
const MAX_CHUNK_SIZE: u32 = 100 * 1024 * 1024;

/// RIFFチャンク情報
#[derive(Debug, Clone)]
pub struct RiffChunk {
    /// チャンクID（4文字）
    pub id: [u8; 4],
    /// チャンクサイズ（バイト）
    pub size: u32,
    /// チャンクデータの開始位置
    pub data_offset: u64,
}

/// RIFFリーダー
pub struct RiffReader<R: Read + Seek> {
    reader: R,
    file_size: u64,
    bytes_read: u64,
}

impl<R: Read + Seek> RiffReader<R> {
    /// 新しいRIFFリーダーを作成
    ///
    /// RIFFヘッダーと"NIKS"フォーマット識別子を検証します。
    pub fn new(mut reader: R) -> Result<Self> {
        let mut riff_header = [0u8; 4];
        reader.read_exact(&mut riff_header)?;

        if &riff_header != b"RIFF" {
            return Err(ParseError::InvalidRiff);
        }

        let mut size_bytes = [0u8; 4];
        reader.read_exact(&mut size_bytes)?;
        // RIFFファイルサイズ = RIFFヘッダー(8バイト) + ペイロードサイズ
        let riff_payload_size = u64::from(u32::from_le_bytes(size_bytes));
        let file_size = riff_payload_size + 8; // RIFF(4) + size(4)

        let mut format_id = [0u8; 4];
        reader.read_exact(&mut format_id)?;

        if &format_id != b"NIKS" {
            return Err(ParseError::InvalidNiks);
        }

        Ok(RiffReader {
            reader,
            file_size,
            bytes_read: 12, // RIFF (4) + size (4) + NIKS (4)
        })
    }

    /// 次のチャンクを読み取る
    ///
    /// ファイルの終端に達した場合は`None`を返します。
    pub fn next_chunk(&mut self) -> Result<Option<RiffChunk>> {
        if self.bytes_read >= self.file_size {
            return Ok(None);
        }

        let mut chunk_id = [0u8; 4];
        match self.reader.read_exact(&mut chunk_id) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                return Ok(None);
            }
            Err(e) => return Err(e.into()),
        }

        let mut size_bytes = [0u8; 4];
        self.reader.read_exact(&mut size_bytes)?;
        let chunk_size = u32::from_le_bytes(size_bytes);

        let data_offset = self.bytes_read + 8; // +8 for chunk ID and size
        self.bytes_read += 8;

        Ok(Some(RiffChunk {
            id: chunk_id,
            size: chunk_size,
            data_offset,
        }))
    }

    /// チャンクデータを読み取る
    ///
    /// チャンクの位置にシークし、データを読み取ります。
    /// 奇数バイトのチャンクの場合、パディングをスキップします。
    pub fn read_chunk_data(&mut self, chunk: &RiffChunk) -> Result<Vec<u8>> {
        // 不正なファイルで巨大なチャンクサイズ（例: 4GB）を指定された場合、
        // メモリを大量に確保しようとしてシステムがクラッシュする可能性がある。
        // 通常のNKSFファイルのチャンクは数十MB以下なので、100MBを上限とする。
        if chunk.size > MAX_CHUNK_SIZE {
            return Err(ParseError::InvalidNiks);
        }

        self.reader.seek(SeekFrom::Start(chunk.data_offset))?;

        let mut data = vec![0u8; chunk.size as usize];
        self.reader.read_exact(&mut data)?;

        self.bytes_read += u64::from(chunk.size);

        // RIFFフォーマットでは、奇数バイトのチャンクには1バイトのパディングが追加される
        // これをスキップしないと、次のチャンクの読み取り位置がずれる
        if chunk.size % 2 == 1 {
            let mut padding = [0u8; 1];
            if self.reader.read_exact(&mut padding).is_ok() {
                self.bytes_read += 1;
            }
        }

        Ok(data)
    }

    /// 全バイトが読み取られたことを確認
    ///
    /// 未読のバイトが残っている場合、`IncompleteParse`エラーを返します。
    pub fn verify_complete(&self) -> Result<()> {
        if self.bytes_read < self.file_size {
            return Err(ParseError::IncompleteParse(
                (self.file_size - self.bytes_read) as usize,
                self.bytes_read as usize,
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_valid_riff_header() {
        // 有効なRIFFヘッダーを持つモックデータ
        let data = b"RIFF\x10\x00\x00\x00NIKS";
        let cursor = Cursor::new(data);
        let reader = RiffReader::new(cursor);
        assert!(reader.is_ok());
    }

    #[test]
    fn test_invalid_riff_magic() {
        // 不正なマジックナンバー
        let data = b"INVALID\x10\x00\x00\x00NIKS";
        let cursor = Cursor::new(data);
        let reader = RiffReader::new(cursor);
        assert!(matches!(reader, Err(ParseError::InvalidRiff)));
    }

    #[test]
    fn test_invalid_niks_format() {
        // 不正なフォーマット識別子
        let data = b"RIFF\x10\x00\x00\x00INVA";
        let cursor = Cursor::new(data);
        let reader = RiffReader::new(cursor);
        assert!(matches!(reader, Err(ParseError::InvalidNiks)));
    }

    #[test]
    fn test_chunk_reading() {
        // RIFFヘッダー + 1つのチャンク
        let mut data = Vec::new();
        data.extend_from_slice(b"RIFF");
        data.extend_from_slice(&20u32.to_le_bytes()); // file size - 8
        data.extend_from_slice(b"NIKS");
        data.extend_from_slice(b"TEST");
        data.extend_from_slice(&8u32.to_le_bytes()); // chunk size
        data.extend_from_slice(b"12345678"); // chunk data

        let cursor = Cursor::new(data);
        let mut reader = RiffReader::new(cursor).unwrap();

        let chunk = reader.next_chunk().unwrap();
        assert!(chunk.is_some());

        let chunk = chunk.unwrap();
        assert_eq!(&chunk.id, b"TEST");
        assert_eq!(chunk.size, 8);

        let chunk_data = reader.read_chunk_data(&chunk).unwrap();
        assert_eq!(&chunk_data, b"12345678");
    }

    #[test]
    fn test_verify_complete() {
        let data = b"RIFF\x04\x00\x00\x00NIKS";
        let cursor = Cursor::new(data);
        let reader = RiffReader::new(cursor).unwrap();

        // 全バイトが読み取られている
        let result = reader.verify_complete();
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_incomplete() {
        let mut data = Vec::new();
        data.extend_from_slice(b"RIFF");
        data.extend_from_slice(&20u32.to_le_bytes()); // file size indicates more data
        data.extend_from_slice(b"NIKS");

        let cursor = Cursor::new(data);
        let reader = RiffReader::new(cursor).unwrap();

        // 未読のバイトが残っている
        let result = reader.verify_complete();
        assert!(matches!(result, Err(ParseError::IncompleteParse(_, _))));
    }
}
