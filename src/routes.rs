use crate::models::{Status,Message};

use actix_web::{get, HttpResponse, Responder};


pub async fn index() -> impl Responder {
    HttpResponse::Ok().json(Message { message: String::from("Index") })
}

pub async fn health() -> impl Responder {
    HttpResponse::Ok().json(Status { status: String::from("ok") })
}

#[get("/hello")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().json(Message { message: String::from("Hello") })
}

