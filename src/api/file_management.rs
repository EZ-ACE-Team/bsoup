use crate::model::admin::{FileDataInfo, FileDeleteInfo, ReqDeleteFileList, ReqFile, ReqSaveMD};
use actix_web::{post, web, HttpResponse, Responder};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{fs, panic};

#[post("/admin/getReadfile")]
pub async fn get_readfile(body: web::Json<ReqFile>) -> impl Responder {
    let path = body.request_path.to_owned();
    let file_name = &body.file_name;

    let new_path = Path::new(&path).join(file_name);

    let f = fs::read_to_string(new_path).expect("Should have been able to read the file");

    HttpResponse::Ok().body(f)
}

#[post("/admin/saveMarkdown")]
pub async fn save_markdown(body: web::Json<ReqSaveMD>) -> impl Responder {
    let path = body.request_path.to_owned();
    let file_name = body.file_name.to_owned();

    let new_path = Path::new(&path).join(&file_name);

    let mut f = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(new_path)
        .unwrap();

    // let mut f = fs::File::open(new_path).expect("Should have been able to read the file");
    f.write_all(body.markdown.as_bytes()).expect("write failed");

    println!("file save success");

    HttpResponse::Ok()
}

#[post("/admin/removeFile")]
pub async fn remove_file(body: web::Json<ReqFile>) -> impl Responder {
    let path = body.request_path.to_owned();
    let file_name = body.file_name.to_owned();

    let new_path = Path::new(&path).join(&file_name);

    fs::remove_file(new_path).expect("could not remove file");

    println!("file remove success");

    HttpResponse::Ok()
}

#[post("/admin/removeFileList")]
pub async fn remove_file_list(files: web::Json<Vec<FileDeleteInfo>>) -> impl Responder {
    for file in files.iter() {
        let file_path = Path::new(&file.file_path).join(&file.name);

        println!("{:?}", file_path);

        let f = if file.file_type == "directory" {
            fs::remove_dir(&file_path)
        } else {
            fs::remove_file(&file_path)
        };
    }

    HttpResponse::Ok()
}
