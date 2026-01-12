use serde::{Deserialize, Serialize};

/// .nksfファイルの完全な解析結果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NksfFile {
    /// メタデータ（NISIチャンク）
    pub metadata: NisiMetadata,
    /// パラメータデータ（NICAチャンク）
    pub parameters: NicaData,
    /// その他の未知のチャンク（完全なバイト解析のため）
    pub unknown_chunks: Vec<UnknownChunk>,
}

/// NISIチャンクのメタデータ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NisiMetadata {
    /// 内部データ
    #[serde(rename = "__ni_internal")]
    pub ni_internal: NiInternal,
    /// 作者名
    pub author: String,
    /// バンクチェーン
    pub bankchain: Vec<String>,
    /// キャラクタータグ
    pub characters: Vec<String>,
    /// コメント
    pub comment: String,
    /// デバイスタイプ
    #[serde(rename = "deviceType")]
    pub device_type: String,
    /// モード
    pub modes: Vec<String>,
    /// プリセット名
    pub name: String,
    /// タイプ
    pub types: Vec<Vec<String>>,
    /// UUID
    pub uuid: String,
    /// ベンダー名
    pub vendor: String,
}

/// 内部データ（詳細構造は調査中）
/// 現時点では全データを保持するためにserde_json::Valueを使用
pub type NiInternal = serde_json::Value;

/// NICAチャンクのパラメータデータ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NicaData {
    /// パラメータリストと追加データ
    pub ni8: Vec<serde_json::Value>,
}

/// パラメータ情報
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    /// 自動命名フラグ
    pub autoname: bool,
    /// パラメータID
    pub id: u32,
    /// パラメータ名
    pub name: String,
    /// 可視性フラグ
    pub vflag: bool,
}

/// 未知のチャンク（完全なバイト解析のため保持）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnknownChunk {
    /// チャンクID（4文字）
    pub id: String,
    /// チャンクのバージョン（存在する場合）
    pub version: Option<u32>,
    /// 生データ
    pub data: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parameter_creation() {
        let param = Parameter {
            autoname: true,
            id: 0,
            name: "Test Parameter".to_string(),
            vflag: false,
        };
        assert_eq!(param.id, 0);
        assert_eq!(param.name, "Test Parameter");
    }

    #[test]
    fn test_unknown_chunk_creation() {
        let chunk = UnknownChunk {
            id: "TEST".to_string(),
            version: Some(1),
            data: vec![0x01, 0x02, 0x03],
        };
        assert_eq!(chunk.id, "TEST");
        assert_eq!(chunk.version, Some(1));
        assert_eq!(chunk.data.len(), 3);
    }

    #[test]
    fn test_nisi_metadata_serialize() {
        let metadata = NisiMetadata {
            ni_internal: serde_json::Value::Null,
            author: "Test Author".to_string(),
            bankchain: vec!["Bank1".to_string()],
            characters: vec!["Char1".to_string()],
            comment: "Test Comment".to_string(),
            device_type: "INST".to_string(),
            modes: vec!["Mode1".to_string()],
            name: "Test Preset".to_string(),
            types: vec![vec!["Type1".to_string()]],
            uuid: "test-uuid".to_string(),
            vendor: "Test Vendor".to_string(),
        };

        // シリアライズできることを確認
        let serialized = serde_json::to_string(&metadata);
        assert!(serialized.is_ok());
    }
}
