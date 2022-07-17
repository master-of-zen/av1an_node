use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use local_ip_address::local_ip;
use std::fmt::format;
use std::net::IpAddr;
use std::sync::Mutex;

struct AppState {
    servers: Mutex<Vec<IpAddr>>,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let my_local_ip = local_ip().unwrap();
    let ip = format!("{:}:8080", my_local_ip.to_string());

    println!("Starting at: {:}", ip);

    let sr = web::Data::new(AppState {
        servers: Mutex::new(vec![]),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(sr.clone())
            .service(health)
            .service(chunks)
            .service(get_chunk)
            .service(connect)
    })
    .bind(ip)?
    .run()
    .await
}

#[get("/chunks")]
async fn chunks() -> impl Responder {
    println!("All chunks");
    format!("All chunks")
}

#[get("/chunks/{id}")]
async fn get_chunk(id: web::Path<String>) -> impl Responder {
    println!("Get chunk number {id}");
    format!("Get chunk number {id}")
}

#[get("/connect")]
pub async fn connect(req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
    if let Some(val) = req.peer_addr() {
        let mut servers = data.servers.lock().unwrap();
        servers.push(val.ip());
        println!("Registered server {:?}", val.ip());
    };

    HttpResponse::Ok()
}

#[get("/")]
pub async fn health(data: web::Data<AppState>) -> impl Responder {
    let servers = data.servers.lock().unwrap();
    let resp = format!("Status Ok, All servers: {:?}", servers);

    println!("{}", resp);

    format!("{}", resp)
}
