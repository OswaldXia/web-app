use crate::auth::jwt::JwtToken;
use crate::database::establish_connection;
use crate::json_serialization::login::Login;
use crate::models::user::user::User;
use crate::schema::users;
use actix_web::{web, HttpResponse};
use diesel::prelude::*;

pub async fn login(credentials: web::Json<Login>) -> HttpResponse {
    let user_name = &credentials.user_name;
    let password = &credentials.password;
    
    let connection = &mut establish_connection();

    let user_list = users::table
        .filter(users::columns::username.eq(user_name))
        .load::<User>(connection)
        .unwrap();

    if user_list.is_empty() {
        HttpResponse::NotFound().await.unwrap()
    } else if user_list.len() > 1 {
        HttpResponse::Conflict().await.unwrap()
    } else if user_list[0].verify(password) {
        let user_token = JwtToken::encode(user_list[0].id);
        HttpResponse::Ok().append_header(("user-token", user_token)).await.unwrap()
    } else {
        HttpResponse::Unauthorized().await.unwrap()
    }
}
