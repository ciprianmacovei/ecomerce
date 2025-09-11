use actix_web::{Responder, get, web};

use crate::controllers::products;

#[get("/all")]
async fn get_all_products() -> impl Responder {
    products::controller::all()
}

#[get("/{id}")]
async fn get_product(id: web::Path<i64>) -> impl Responder {
    let id: i64 = id.into_inner();
    products::controller::product(id)
}

#[get("/all/{page}/{page_size}")]
async fn get_products_page(page: web::Path<i32>, page_size: web::Path<i32>) -> impl Responder {
    let page: i32 = page.into_inner();
    let page_size: i32 = page_size.into_inner();
    products::controller::products_page(page, page_size)
}

fn products_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(get_all_products)
        .service(get_product)
        .service(get_products_page);
}

pub(crate) fn main_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/products") // mount all under `/users`
            .configure(products_config)
    );
}

