use crate::model::document::{DocsType, Document, ExceptionCode, Root};
use actix_web::http::StatusCode;
use actix_web::{post, web, HttpResponse, Responder};
use std::fs;
use std::fs::{DirEntry, ReadDir};
use std::path::Path;
use walkdir::WalkDir;

#[post("/admin/readDocumentList")]
pub async fn read_document_list(body: web::Json<DocsType>) -> impl Responder {
    let path = "develop-center-md";
    let sub_path = "document";

    let root_path = Path::new(&path).join(&body.docs_type);

    let mut root = Root { document: None };

    let mut main_docs = Document::default();

    if let Ok(entries) = fs::read_dir(&root_path) {
        main_docs = entries_in_files(entries, path, sub_path);
        main_docs.title = String::from("문서");
        main_docs.path = sub_path.to_string();
    } else {
        let mut exception = ExceptionCode::default();
        exception.err_code = StatusCode::NOT_FOUND.as_u16();
        exception.err_message = StatusCode::NOT_FOUND.to_string();
        return HttpResponse::Ok().json(exception);
    }

    root.document = Some(main_docs);

    return HttpResponse::Ok().json(root);
}

pub fn entries_in_files(entries: ReadDir, root: &str, path: &str) -> Document {
    let mut root_docs = Document::default();

    for entry in entries.flatten() {
        let mut sub_docs = Document::default();
        if entry.file_type().unwrap().is_file()
            && entry.file_name().to_str().unwrap().contains("md")
        {
            root_docs
                .menu
                .push(entry.file_name().to_str().unwrap().to_string());
        }
        if entry.file_type().unwrap().is_dir() {
            if let Ok(ent) = fs::read_dir(entry.path()) {
                sub_docs = entries_in_files(ent, root, path);
                let path_parent = entry.path().parent().unwrap().to_str().unwrap().to_string();
                sub_docs.title =
                    entry.path().to_str().unwrap()[path_parent.len() + 1..].to_string();
                sub_docs.path = entry.path().to_str().unwrap()[root.len() + 1..].to_string();
            }

            root_docs.submenu.push(sub_docs);
        }
    }

    root_docs
}
