use super::utils::return_state;
use crate::processes::process_input;
use crate::schema::to_do;
use crate::state::read_file;
use crate::to_do::to_do_factory;
use crate::{database::establish_connection, json_serialization::to_do_item::ToDoItem};
use actix_web::{web, HttpResponse};
use diesel::prelude::*;

pub async fn edit(to_do_item: web::Json<ToDoItem>) -> HttpResponse {
    edit_db(to_do_item).await
}

#[allow(dead_code)]
pub async fn edit_db(to_do_item: web::Json<ToDoItem>) -> HttpResponse {
    let title = &to_do_item.title;
    let mut connection = establish_connection();
    let result = to_do::table.filter(to_do::columns::title.eq(title));
    diesel::update(result)
        .set(to_do::columns::status.eq("done"))
        .execute(&mut connection)
        .unwrap();
    HttpResponse::Ok().json(return_state())
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
            HttpResponse::Ok().json(return_state())
        } else {
            HttpResponse::BadRequest().json(format!("{} not accepted", status))
        }
    } else {
        HttpResponse::NotFound().json(format!("{} not in state", title))
    }
}
