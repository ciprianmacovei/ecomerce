pub mod user;

use actix_web::{web};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user") // mount all under `/user`
            .configure(user::main_config)
    );
}