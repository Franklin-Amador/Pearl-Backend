// This file sets up the Actix web server and configures the routes.

use actix_web::{web, App, HttpServer};
mod handlers {
    use actix_web::{HttpResponse, Responder};

    pub async fn get_index() -> impl Responder {
        HttpResponse::Ok().body("Hello, world!")
    }
}
pub use handlers::get_index;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}