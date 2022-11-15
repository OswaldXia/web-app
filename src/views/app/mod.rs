mod content_loader;
mod items;
mod login;
mod logout;

use super::path::Path;
use actix_web::web::{self, ServiceConfig};

pub fn app_factory(cfg: &mut ServiceConfig) {
    //  do not have any URL prefix because this is our main URL get functions for the user.
    let base_path = Path {
        prefix: "/".to_string(),
        backend: false
    };

    cfg
        .route(
            &base_path.define(""), 
            web::get().to(items::items))
        .route(
            &base_path.define("login"), web::get().to(login::login))
        .route(
            &base_path.define("logout"), 
            web::get().to(logout::logout));
}
