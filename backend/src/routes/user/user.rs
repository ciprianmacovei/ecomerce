use actix_web::{get, post, web, Responder};
use crate::controllers::user::controller::{register, UserRequest};

use crate::lib;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}

#[get("/hello1/{name}")]
async fn greet1(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}

#[post("/register")]
async fn user_register(user: web::Json<UserRequest>) -> impl Responder {
    // Call the _register function from the controller
    let mut connection: diesel::PgConnection = lib::establish_pg_connection();
    register(&mut connection, user.into_inner()).await
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(greet1)
        .service(greet)
        .service(user_register);
}

pub(crate) fn main_config(cfg: &mut web::ServiceConfig) {
    cfg.configure(config);
}

