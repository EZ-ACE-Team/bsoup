use std::fs;
use actix_web::{web, post, HttpResponse, Responder};
use std::path::{Path};
use crate::model::admin::{ReqFile, ReqPath};
use std::fs::File;

#[post("/admin/getReadfile")]
pub async fn get_readfile(body: web::Json<ReqFile>) -> impl Responder {
    let path = body.request_path.to_owned();
    let file_name = &body.file_name;

    let new_path = Path::new(&path).join(file_name);

    let mut f = fs::read_to_string(new_path).expect("Should have been able to read the file");

    println!("{:?}", f);

    HttpResponse::Ok().body(f)
}