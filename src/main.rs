mod api;
mod model;

use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(api::directory::get_document_list)
            .service(api::file_management::get_readfile)
            .service(api::file_management::save_markdown)
            .service(api::file_management::remove_file)
            .service(api::directory::create_dir)
            .service(api::uploader::text_file_uploader)
            .service(api::file_management::remove_file_list)
            .service(api::document::read_document_list)
            .route("/", web::get().to(HttpResponse::Ok))
    })
    .workers(4)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
