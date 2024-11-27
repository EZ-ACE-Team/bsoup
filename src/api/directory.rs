extern crate walkdir;
extern crate chrono;
use crate::model;

use std::ffi::OsStr;
use std::fs;
use actix_web::{web, post, HttpResponse, Responder};
use walkdir::WalkDir;
use std::path::{Path};
use chrono::{TimeZone, Utc};
use model::admin::{ReqPath, FileDataInfo};
use crate::model::admin::ReqFile;

// path ) 디렉토리 리스트 추출
pub fn list_all_files<T: AsRef<Path>>(path: T) -> Vec<FileDataInfo> {
    let mut files = Vec::new();

    // path 디렉토리의 모든 파일과 하위 디렉토리를 재귀 탐색
    let walker = WalkDir::new(&path);

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
                file_type: None,
                file_path: None
                };
            let file_path = entry.file_name().to_str().unwrap().to_string();

            match entry.path().extension().and_then(OsStr::to_str) {
                Some(_) => {
                    let file_ms = get_modified_secs(
                        entry.path().to_str().unwrap()
                    ).to_string().parse::<i64>().unwrap();

                    let datetime = Utc.timestamp_opt(file_ms, 0).single()
                        .expect("not validate type").to_string();

                    file_info.upload = Some(datetime);
                    file_info.file_type = Some(FileDataInfo::get_extension(entry.path()));
                }
                None => {
                    file_info.file_type = Some(FileDataInfo::get_extension(entry.path()));
                }
            }

            file_info.name = Some(file_path);
            file_info.file_path = Some(path.as_ref().to_owned());

            files.push(file_info);
        }
    }

    files.sort_by_key(|a| a.upload.is_some());

    return files;
}

// 파일 마지막 수정 시간 확인
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

// 파일 필터링
pub fn filtered_dir_list<'a>(files: Vec<FileDataInfo>, dir_path: &str, sub_path: &str) -> Vec<FileDataInfo> {
    println!("path : {:?}, sub_path : {:?}",dir_path, sub_path);
    if sub_path.len() > 0 && sub_path != ".." {
        files.iter().cloned().map(|mut x| {
            if x.name == Some(sub_path.to_string()) {
                x.name = Some("..".to_string());
            }
            x
        }).collect()
    } else {
        files.into_iter().filter(|p| &p.name.as_ref().unwrap().to_string() != dir_path)
            .collect()
    }
}

#[post("/admin/getDocumentList")]
pub async fn get_document_list(body: web::Json<ReqPath>) -> impl Responder {
    let dir_path = "develop-center-md";
    let sub_dir = &body.request_path;
    let path = if sub_dir == ".." {
        Path::new(&body.path).parent().map(|p| p.to_path_buf())
    } else {
        Some(Path::new(&dir_path).join(sub_dir))
    };

    let files: Vec<FileDataInfo> = list_all_files(path.unwrap());

    HttpResponse::Ok().json(filtered_dir_list(files, &dir_path, &sub_dir))
}

#[post("/admin/createDirectory")]
pub async fn create_dir(body: web::Json<ReqFile>) -> impl Responder {
    let path = if body.request_path == "" {
        "develop-center-md".to_string()
    } else {
        body.request_path.to_owned()
    };

    let dir_name = body.file_name.to_owned();

    let new_path = Path::new(&path).join(&dir_name);

    let mut f = fs::create_dir(new_path).expect("could not create directory");

    println!("directory crate success");

    HttpResponse::Ok()
}