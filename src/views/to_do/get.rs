// creating a GET view that returns all our to do items

use super::utils::{return_state_db, return_state_json};
use crate::auth::jwt::JwtToken;
use actix_web::{HttpRequest, Responder};

#[allow(dead_code)]
pub async fn get_db(request: HttpRequest) -> impl Responder {
    let user_id = JwtToken::decode_from_request(&request).unwrap().user_id;
    return_state_db(user_id)
}

#[allow(dead_code)]
pub async fn get_json() -> impl Responder {
    return_state_json()
}
