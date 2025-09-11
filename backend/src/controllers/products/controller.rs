use actix_web::{web, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Product {
    id: i64,
    image: String,
    title: String,
    content: String,
    price: i32,
    currency: String,
}

#[derive(Serialize)]
struct PaginatedProduct {
    pub products: Vec<Product>,
    pub page: i32,
    pub page_size: i32,
}


pub fn all() -> impl Responder {
    web::Json(vec!["asdasd".to_string(), "asdasd".to_string()])
}

pub fn product(id: i64) -> impl Responder {
    web::Json(
        Product{
            id,
            image: "".to_string(),
            title: "First product".to_string(),
            content: "First product content".to_string(),
            price: 20,
            currency: "EUR".to_string(),
        }
    )
}

pub fn products_page(page: i32, page_size: i32) -> impl Responder {
    let products: Vec<Product> = vec![
        Product {
            id: 1,
            image: "".to_string(),
            title: "First product".to_string(),
            content: "First product content".to_string(),
            price: 20,
            currency: "EUR".to_string(),
        },
    ];

    web::Json(PaginatedProduct {
        products,
        page,
        page_size,
    })
}
