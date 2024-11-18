extern crate walkdir;
extern crate chrono;

use std::ffi::OsStr;
use std::fs;
use std::fs::metadata;
use actix_web::{get, HttpResponse, Responder};
use walkdir::WalkDir;
use std::path::Path;
use std::time::{SystemTime};
use actix_web::dev::ResourcePath;
use serde;
use chrono::{DateTime, Duration, TimeZone, Utc};

#[derive(Debug, Clone, serde::Serialize)]
pub struct FileDataInfo {
    name: Option<String>,
    description: Option<String>,
    upload: Option<String>,
    file_type: Option<String>
}

pub fn list_all_files<P: AsRef<Path>>(path: P) -> Vec<FileDataInfo> {
    let mut files = Vec::new();

    // path 디렉토리의 모든 파일과 하위 디렉토리를 재귀 탐색
    let walker = WalkDir::new(path);

    // walker 결과를 반복자로 변환
    let iterator = walker.into_iter();

    // iterator 각 항목 ('e')에 대해 오류 발생 필터링 후 성공 항목만 반환
    let filtered_iterator = iterator.filter_map(|e| e.ok());

    // 필터링된 결과를 entry로 하나씩 꺼내 처리
    for entry in filtered_iterator {
        // 파일 유형 확인
        if entry.file_type().is_file() || entry.file_type().is_dir() {
            let mut file_info = FileDataInfo {
                name: None,
                description: None,
                upload: None,
                file_type: None
            };

            let file_path = entry.file_name().to_str().unwrap().to_string();
            let file_meta = entry.metadata().unwrap();

            match entry.path().extension().and_then(OsStr::to_str) {
                Some(_) => {
                    let file_ms = get_modified_secs(
                        entry.path().to_str().unwrap()
                    ).to_string().parse::<i64>().unwrap();

                    let datetime = Utc.timestamp_opt(file_ms, 0).single()
                        .expect("not validate type").to_string();

                    file_info.upload = Some(datetime);
                    file_info.file_type = Some(get_extension(entry.path()));
                }
                None => {
                    file_info.file_type = Some(get_extension(entry.path()));
                }
            }

            file_info.name = Some(file_path);

            files.push(file_info);
        }
    }

    return files;
}

fn get_extension(entry: &std::path::Path) -> String {
    if (entry.is_dir()) {
        "directory".to_string()
    } else {
        entry.extension().and_then(|ext| ext.to_str()).map(|ext_str| ext_str.to_string()).unwrap()
    }
}

pub fn get_modified_secs(file: &str) -> usize {
    let modified_date = fs::metadata(file).expect("Need metadate");

    let secs = modified_date
        .modified()
        .expect("Need modified date")
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Need duration")
        .as_secs();

    secs.try_into().unwrap()
}

#[get("/admin/getDocumentList")]
pub async fn get_markdown() -> impl Responder {
    let files: Vec<FileDataInfo> = list_all_files("develop-center-md");

    HttpResponse::Ok().json(files)
}