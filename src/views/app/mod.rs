mod items;
mod content_loader;

use super::path::Path;
use actix_web::web::{self, ServiceConfig};

pub fn app_factory(cfg: &mut ServiceConfig) {
    //  do not have any URL prefix because this is our main URL get functions for the user. 
    let base_path = Path {
        prefix: "/".to_string(),
    };
    cfg.route(
        &base_path.define("".to_string()),
        web::get().to(items::items),
    );
}
