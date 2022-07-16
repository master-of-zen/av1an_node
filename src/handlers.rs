// src/handlers.rs

use actix_web::Responder;

pub async fn get_chunks() -> impl Responder {
    format!("hello from get chunks")
}

pub async fn get_chunk_by_id() -> impl Responder {
    format!("hello from get chunk by id")
}

pub async fn add_chunk() -> impl Responder {
    format!("hello from add chunk")
}

pub async fn delete_chunk() -> impl Responder {
    format!("hello from delete chunk")
}
