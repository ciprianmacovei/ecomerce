// Update the path to your models module as appropriate.
// For example, if models.rs is in src/models.rs:
use crate::models::user::{Users, NewUser}; // Adjust the path based on your project structure
use diesel::{connection, prelude::*};
use actix_web::{Error, HttpResponse, Responder, Result, get, post, web};


fn _login() -> String {
    return "Login".to_string();
}

#[derive(serde::Deserialize)]
pub struct UserRequest {
    user_name: String,
    password: String,
    email: String,
    notification_id: String,
}

pub async fn register(conn: &mut PgConnection, user: UserRequest) -> Result<HttpResponse, Error> {

    use crate::schema::users;

    let new_user: NewUser<'_> = NewUser {
        user_name: &user.user_name,
        password: &user.password,
        email: &user.email,
        notification_id: &user.notification_id,
    };

    match diesel::insert_into(users::table)
        .values(&new_user)
        .returning(Users::as_returning())
        .get_result(conn) {
        Ok(_user) => Ok(HttpResponse::Created().body("User registered")),
        Err(e) => Ok(HttpResponse::InternalServerError().body(format!("Error creating user: {}", e))),
    }
    
}