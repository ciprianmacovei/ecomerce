pub mod products;

use actix_web::{ web };

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/products") // mount all under `/users`
            .configure(products::main_config)
    );
}