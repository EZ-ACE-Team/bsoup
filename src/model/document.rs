use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub document: Option<Document>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    pub title: String,
    pub path: String,
    pub menu: Vec<String>,
    pub submenu: Vec<Document>,
}

#[derive(Serialize, Default)]
pub struct ExceptionCode {
    pub err_code: u16,
    pub err_message: String,
}

#[derive(Serialize, Deserialize)]
pub struct DocsType {
    pub docs_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct DocsContent {
    pub docs_path: String,
}
