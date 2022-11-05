use super::utils::return_state;
use crate::{
    json_serialization::to_do_item::ToDoItem, processes::process_input, state::read_file,
    to_do::to_do_factory,
};
use actix_web::{web, HttpResponse};

pub async fn delete(to_do_item: web::Json<ToDoItem>) -> HttpResponse {
    let file_name = "./state.json";
    let state = read_file(file_name);

    let item_title = &to_do_item.title;
    let item_type = &to_do_item.status;

    if let Ok(item) = to_do_factory(item_type, item_title) {
        process_input(item, "delete", &state);
        HttpResponse::Ok().json(return_state())
    } else {
        HttpResponse::BadRequest().json(format!("{} not accepted", item_type))
    }
}
