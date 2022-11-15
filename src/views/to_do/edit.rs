use super::utils::{return_state_db, return_state_json};
use crate::auth::jwt::JwtToken;
use crate::processes::process_input;
use crate::schema::to_do;
use crate::state::read_file;
use crate::to_do::to_do_factory;
use crate::{database::establish_connection, json_serialization::to_do_item::ToDoItem};
use actix_web::{web, HttpRequest, HttpResponse};
use diesel::prelude::*;

#[allow(dead_code)]
pub async fn edit_db(to_do_item: web::Json<ToDoItem>, request: HttpRequest) -> HttpResponse {
    let title = &to_do_item.title;
    let user_id = JwtToken::decode_from_request(&request).unwrap().user_id;
    let connection = &mut establish_connection();
    let result = to_do::table
        .filter(to_do::columns::title.eq(title))
        .filter(to_do::columns::user_id.eq(user_id));
    diesel::update(result)
        .set(to_do::columns::status.eq("done"))
        .execute(connection)
        .unwrap();
    HttpResponse::Ok().json(return_state_db(user_id))
}

#[allow(dead_code)]
pub async fn edit_json(to_do_item: web::Json<ToDoItem>) -> HttpResponse {
    let file_name = "./state.json";
    let state = read_file(file_name);
    let title = &to_do_item.title;

    if let Some(result) = state.get(title) {
        let status = result.to_string().replace('\"', "");
        if let Ok(item) = to_do_factory(&status, title) {
            if status != to_do_item.status {
                process_input(item, "edit", &state);
            }
            HttpResponse::Ok().json(return_state_json())
        } else {
            HttpResponse::BadRequest().json(format!("{} not accepted", status))
        }
    } else {
        HttpResponse::NotFound().json(format!("{} not in state", title))
    }
}
