use crate::to_do::structs::base::Base;
use crate::to_do::ItemTypes;
use actix_web::{body::BoxBody, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
pub struct ToDoItems {
    pub pending_items: Vec<Base>,
    pub done_items: Vec<Base>,
    pub pending_item_count: i8,
    pub done_item_count: i8,
}

impl ToDoItems {
    pub fn new(input_items: Vec<ItemTypes>) -> ToDoItems {
        let mut pending_items = vec![];
        let mut done_items = vec![];

        for item in input_items {
            match item {
                ItemTypes::Pending(packed_item) => pending_items.push(packed_item.super_struct),
                ItemTypes::Done(packed_items) => done_items.push(packed_items.super_struct),
            }
        }

        let pending_item_count = pending_items.len() as i8;
        let done_item_count = done_items.len() as i8;

        ToDoItems {
            pending_items,
            done_items,
            pending_item_count,
            done_item_count,
        }
    }
}

// Implement Responder for ToDoItems, so it can be directly returned in a view.
impl Responder for ToDoItems {
    type Body = BoxBody;
    // The respond_to function is fired when the view function is returned.
    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type("application/json")
            .body(body)
    }
}
