mod api;

use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||
        App::new()
            .service(api::directory::get_markdown)
            .route("/", web::get()
                .to(HttpResponse::Ok)))
        .workers(4).bind(("127.0.0.1", 8080))?
        .run()
        .await
}