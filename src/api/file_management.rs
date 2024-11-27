use std::fs;
use actix_web::{web, post, HttpResponse, Responder};
use std::path::{Path};
use crate::model::admin::{ReqFile, ReqPath, ReqSaveMD};
use std::fs::{File, OpenOptions};
use std::io::Write;

#[post("/admin/getReadfile")]
pub async fn get_readfile(body: web::Json<ReqFile>) -> impl Responder {
    let path = body.request_path.to_owned();
    let file_name = &body.file_name;

    let new_path = Path::new(&path).join(file_name);

    let mut f = fs::read_to_string(new_path).expect("Should have been able to read the file");

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

    let mut f = fs::remove_file(new_path).expect("could not remove file");

    println!("file remove success");

    HttpResponse::Ok()
}