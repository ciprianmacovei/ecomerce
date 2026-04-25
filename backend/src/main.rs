use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use dotenvy::dotenv;
use std::env;

mod controllers;
mod routes;
mod schema;
mod models;
#[path = "lib.rs"]
mod lib;


fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")    // prefix all routes with /api
            .configure(routes::user::config)
            .configure(routes::products::config)
    );
}

#[get("/api")]
async fn api_root() -> impl Responder {
    HttpResponse::Ok().body("Hello from /api")
}


#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port: u16 = env::var("APPLICATION_PORT").unwrap().parse().unwrap();

    println!("Application run on port {}", &port);

    HttpServer::new(move || {
        App::new()
        .service(api_root)
        .configure(config)
    })
        .bind(("0.0.0.0", port))?
        .run()
        .await
}

