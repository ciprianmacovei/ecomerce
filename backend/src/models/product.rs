use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::products)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Products {
    id: i32,
    image: String,
    title: String,
    content: String,
    price: f32,
    currency: String,
}

