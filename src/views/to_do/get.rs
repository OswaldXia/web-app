// creating a GET view that returns all our to do items

use super::utils::return_state;
use actix_web::Responder;

pub async fn get() -> impl Responder {
    return_state()
}
