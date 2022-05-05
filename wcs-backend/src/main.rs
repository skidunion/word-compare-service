mod routes;

use std::io;
use actix_web::{App, HttpServer, web};
use actix_files as fs;

use crate::routes as route;

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/api/v1/words/compare", web::get().to(route::words_v1_api::get_similarity))
            .service(fs::Files::new("/", "wcs-frontend/")
                .index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run().await
}
