use std::path::{PathBuf};
use serde;

#[derive(Debug, Clone, serde::Serialize)]
pub struct FileDataInfo {
    pub name: Option<String>,
    pub description: Option<String>,
    pub upload: Option<String>,
    pub file_type: Option<String>,
    pub file_path: Option<PathBuf>
}

impl FileDataInfo {
    pub fn get_extension(entry: &std::path::Path) -> String {
        if entry.is_dir() {
            "directory".to_string()
        } else {
            entry.extension().and_then(|ext| ext.to_str()).map(|ext_str| ext_str.to_string()).unwrap()
        }
    }
}

#[derive(serde::Deserialize)]
pub struct ReqPath {
    pub request_path: String,            // 메인
    pub path: String,
}

#[derive(serde::Deserialize)]
pub struct ReqFile {
    pub request_path: String,
    pub file_name: String
}

#[derive(serde::Deserialize)]
pub struct ReqSaveMD {
    pub markdown: String,
    pub request_path: String,
    pub file_name: String
}