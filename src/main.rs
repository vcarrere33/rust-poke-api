use actix_web::{web, App, HttpServer};
mod hello;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    HttpServer::new(|| {
        App::new()
            .service(hello::hello)
            .route("/hey", web::get().to(hello::manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
