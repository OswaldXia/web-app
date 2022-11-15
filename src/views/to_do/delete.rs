use super::utils::{return_state_db, return_state_json};
use crate::{
    auth::jwt::JwtToken, database::establish_connection, json_serialization::to_do_item::ToDoItem,
    processes::process_input, schema::to_do, state::read_file, to_do::to_do_factory,
};
use actix_web::{web, HttpRequest, HttpResponse};
use diesel::prelude::*;

#[allow(dead_code)]
pub async fn delete_db(to_do_item: web::Json<ToDoItem>, request: HttpRequest) -> HttpResponse {
    let title = &to_do_item.title;
    let user_id = JwtToken::decode_from_request(&request).unwrap().user_id;
    let connection = &mut establish_connection();

    let items = to_do::table
        .filter(to_do::columns::title.eq(title))
        .filter(to_do::columns::user_id.eq(user_id));

    diesel::delete(items).execute(connection).unwrap();

    HttpResponse::Ok().json(return_state_db(user_id))
}

#[allow(dead_code)]
pub async fn delete_json(to_do_item: web::Json<ToDoItem>) -> HttpResponse {
    let file_name = "./state.json";
    let state = read_file(file_name);

    let item_title = &to_do_item.title;
    let item_type = &to_do_item.status;

    if let Ok(item) = to_do_factory(item_type, item_title) {
        process_input(item, "delete", &state);
        HttpResponse::Ok().json(return_state_json())
    } else {
        HttpResponse::BadRequest().json(format!("{} not accepted", item_type))
    }
}
