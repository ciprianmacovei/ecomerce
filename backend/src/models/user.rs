use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Users {
    id: i32,
    user_name: String,
    password: String,
    email: String,
    notification_id: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser<'a> {
    pub user_name: &'a str,
    pub password: &'a str,
    pub email: &'a str,
    pub notification_id: &'a str,
}