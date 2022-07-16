use actix_web::{get, web, App, HttpServer, Responder};

mod handlers;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Starting at 127.0.0.1:8080");

    // Start http server
    HttpServer::new(move || {
        App::new()
            .route("/chunks", web::get().to(handlers::get_chunks))
            .route("/chunks/{id}", web::get().to(handlers::get_chunk_by_id))
            .route("/chunks", web::post().to(handlers::add_chunk))
            .route("/chunks/{id}", web::delete().to(handlers::delete_chunk))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
