use actix_web::{get, web, App, HttpServer, Responder};
use local_ip_address::local_ip;

mod handlers;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let my_local_ip = local_ip().unwrap();
    let ip = format!("{:}:8080", my_local_ip.to_string());
    println!("Starting at: {:}", ip);

    // Start http server
    HttpServer::new(move || {
        App::new()
            .route("/chunks", web::get().to(handlers::get_chunks))
            .route("/chunks/{id}", web::get().to(handlers::get_chunk_by_id))
            .route("/chunks", web::post().to(handlers::add_chunk))
            .route("/chunks/{id}", web::delete().to(handlers::delete_chunk))
    })
    .bind(ip)?
    .run()
    .await
}
