use actix_web::{post, HttpResponse, Responder};
use actix_multipart::form::{MultipartForm};
use std::fs;
use std::io::{Read, Write};
use crate::model::upload::{UploadForm};

#[post("/admin/fileUploadTest")]
pub async fn text_file_uploader(MultipartForm(form): MultipartForm<UploadForm>) -> impl Responder {
    println!(
        "Uploaded file path {}, with size: {}", form.path_info.name, form.file_info.size
    );

    let dir_path = if &form.path_info.path == "" {
        "develop-center-md"
    } else {
        &form.path_info.path
    };

    let file_name = if &form.path_info.name == "" {
        &form.file_info.file_name.unwrap()
    } else {
        &form.path_info.name
    };

    // dir_path.to_string().push_str(file_name);

    let test_dir = format!("{dir_path}/{file_name}");

    let temp_file = fs::File::open(&form.file_info.file.path());
    let mut buffer = Vec::new();
    temp_file.unwrap().read_to_end(&mut buffer);

    let output_file = fs::File::create(&test_dir);
    output_file.unwrap().write_all(&buffer);

    HttpResponse::Ok()
}