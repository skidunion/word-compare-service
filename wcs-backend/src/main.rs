mod routes;
mod util;

use std::io;
use actix_web::{App, HttpServer, web};
use actix_files as fs;
use crate::routes as rt;

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/api/v1/similarity", web::get().to(rt::api::get_similarity))
            .service(fs::Files::new("/", "wcs-frontend/")
                .index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run().await
}
