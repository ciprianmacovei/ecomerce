use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::product)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Product {
    id: i32,
    image: String,
    title: String,
    content: String,
    price: f32,
    currency: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    id: i32,
    user_name: String,
    password: String,
    email: String,
    notification_id: String,
}