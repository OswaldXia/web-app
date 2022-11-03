use super::utils::return_state;
use crate::json_serialization::to_do_item::ToDoItem;
use crate::processes::process_input;
use crate::state::read_file;
use crate::to_do::to_do_factory;
use actix_web::{web, HttpResponse};

pub async fn edit(to_do_item: web::Json<ToDoItem>) -> HttpResponse {
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

    // // check state to see if the to do item actually exists.
    // let status;
    // if let Some(result) = state.get(title) {
    //     status = result.to_string().replace('\"', "");
    // } else {
    //     return HttpResponse::NotFound().json(format!("{} not in state", title));
    // }

    // // see if the status is the same as the status that was passed into the view.
    // if status == to_do_item.status {
    //     HttpResponse::Ok().json(return_state())
    // } else if let Ok(item) = to_do_factory(&status, title) {
    //     process_input(item, "edit", &state);
    //     HttpResponse::Ok().json(return_state())
    // } else {
    //     HttpResponse::BadRequest().json(format!("{} not accepted", status))
    // }
}
