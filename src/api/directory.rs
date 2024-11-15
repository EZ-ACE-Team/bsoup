extern crate walkdir;

use std::fs::metadata;
use actix_web::{get, HttpResponse, Responder};
use walkdir::WalkDir;
use std::path::Path;
use std::time::{Duration, SystemTime};
use serde;

#[derive(Debug, Clone, serde::Serialize)]
pub struct FileDataInfo {
    Name: Option<String>,
    Description: Option<String>,
    Upload: Option<Duration>
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
            let mut fileInfo = FileDataInfo {
                Name: None,
                Description: None,
                Upload: None
            };

            let file_path = entry.file_name().to_str().unwrap().to_string();
            let file_meta = entry.metadata().unwrap();

            fileInfo.Name = Some(file_path);

            if let Ok(modifiedd) = file_meta.modified() {
                let datetime = SystemTime::now().duration_since(modifiedd).unwrap();
                let modified = SystemTime::now() - Duration::from_secs(5 * 3600 * 30 * 60);
                let datetime = SystemTime::now().duration_since(modified).unwrap();

                let total_seconds = datetime.as_secs();

                let hours = total_seconds / 3600;
                let minutes = (total_seconds % 3600) / 60;
                let seconds = total_seconds % 60;


                println!("현재 시간 {:?}", modified);
                println!("차이 {:?}",datetime);
                println!("시간 차이는 {}시간 {}분 {}초", hours, minutes, seconds);

                fileInfo.Upload = Some(datetime);
            }

            files.push(fileInfo);
        }
    }

    return files;
}

#[get("/getMarkdown")]
pub async fn get_markdown() -> impl Responder {
    let files: Vec<FileDataInfo> = list_all_files("develop-center-md");

    HttpResponse::Ok().json(files)
}