// to build a create view function.

use crate::database::establish_connection;
use crate::json_serialization::new_user::NewUserSchema;
use crate::models::user::new_user::NewUser;
use crate::schema::users;
use actix_web::{web, HttpResponse};
use diesel::prelude::*;

pub async fn create(new_user: web::Json<NewUserSchema>) -> HttpResponse {
    let connection = &mut establish_connection();

    let new_user = NewUser::new(&new_user.name, &new_user.email, &new_user.password);

    let result = diesel::insert_into(users::table)
        .values(&new_user)
        .execute(connection);

    match result {
        Ok(_) => HttpResponse::Created().await.unwrap(),
        Err(_) => HttpResponse::Conflict().await.unwrap(),
    }
}
