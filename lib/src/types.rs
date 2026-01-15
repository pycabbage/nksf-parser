use serde::{Deserialize, Serialize};

/// .nksfファイルの完全な解析結果
///
/// NKSFファイルは4つのチャンクから構成されます:
/// - NISI: メタデータ（プリセット名、作者、タグ等）
/// - NICA: パラメータアサインメント（マクロコントロールの割り当て）
/// - PLID: プラグインID（VSTマジックナンバー等）
/// - PCHK: プラグインチャンク（Massive Xの全パラメータ値、zlib圧縮されたMessagePackデータ）
///
/// # Examples
///
/// ```no_run
/// use nksf_parser::parse_nksf;
/// use std::path::Path;
///
/// let nksf = parse_nksf(Path::new("preset.nksf"))?;
/// println!("Preset: {}", nksf.metadata.name);
/// println!("Author: {}", nksf.metadata.author);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NksfFile {
    /// メタデータ（NISIチャンク）
    pub metadata: NisiMetadata,
    /// パラメータデータ（NICAチャンク）
    pub parameters: NicaData,
    /// プラグインID（PLIDチャンク）
    pub plugin_id: PlidData,
    /// プラグインチャンク（PCHKチャンク）
    pub plugin_chunk: PchkData,
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
    /// モード（一部のプリセットには存在しない）
    #[serde(default)]
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
/// `現時点では全データを保持するためにserde_json::Valueを使用`
pub type NiInternal = serde_json::Value;

/// PLIDチャンク（Plugin ID）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlidData {
    /// VSTマジックナンバー
    #[serde(rename = "VST.magic")]
    pub vst_magic: u32,

    /// プラグイン名（オプション）
    #[serde(rename = "pluginName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin_name: Option<String>,

    /// プラグインベンダー名（オプション）
    #[serde(rename = "pluginVendor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin_vendor: Option<String>,
}

/// PCHKチャンク（Plugin Chunk）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PchkData {
    /// ヘッダー情報
    pub header: PchkHeader,

    /// 全MessagePack値（268個）
    /// 構造: [name1, count1, data1, name2, count2, data2, ...]
    pub values: Vec<serde_json::Value>,
}

/// PCHKヘッダー（20バイト）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PchkHeader {
    /// バージョン（通常1）
    pub version: u32,

    /// 不明フィールド1（用途未特定、観測値は2）
    pub field1: u32,

    /// 不明フィールド2（用途未特定、観測値は2）
    pub field2: u32,

    /// 圧縮データサイズ（zlib圧縮後のバイト数）
    pub compressed_size: u32,

    /// 不明フィールド3（用途未特定、値は可変）
    pub field3: u32,
}

/// NICAチャンクのパラメータデータ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NicaData {
    /// パラメータリストと追加データ（一部のプリセットには存在しない）
    #[serde(default)]
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
