mod config;
mod models;
mod routes;

use crate::config::fetch_config;
use crate::routes::*;

use env_logger;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use listenfd::ListenFd;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let mut listenfd = ListenFd::from_env();

    let listen_url = format!("{}:{}", fetch_config("LISTEN_ADDR"), fetch_config("LISTEN_PORT"));
    println!("Listening on http://{}", listen_url );

    let mut server = HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(index))
            .route("/health", web::get().to(health))
            .service(hello)
    });
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind(listen_url)?
    };

    server.run().await

}

