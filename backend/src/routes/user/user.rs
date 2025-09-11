use actix_web::{get, web, Responder};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}

#[get("/hello1/{name}")]
async fn greet1(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(greet1)
        .service(greet);
}

pub(crate) fn main_config(cfg: &mut web::ServiceConfig) {
    cfg.configure(config);
}

