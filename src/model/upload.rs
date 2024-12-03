use actix_multipart::form::{json::Json as MPJson, tempfile::TempFile, MultipartForm};
use serde::*;

#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub(crate) path: String,
    pub(crate) name: String
}

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    #[multipart(limit = "100MB")]
    pub(crate) file_info: TempFile,
    pub(crate) path_info: MPJson<Metadata>
}