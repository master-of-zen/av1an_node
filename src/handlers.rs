// src/handlers.rs

use actix_web::Responder;

pub async fn get_chunks() -> impl Responder {
    println!("get chunks called");
    format!("hello from get chunks")
}

pub async fn get_chunk_by_id() -> impl Responder {
    println!("get chunk called");
    format!("hello from get chunk by id")
}

pub async fn add_chunk() -> impl Responder {
    println!("add chunk called");
    format!("hello from add chunk")
}

pub async fn delete_chunk() -> impl Responder {
    println!("delete chunk called");
    format!("hello from delete chunk")
}
