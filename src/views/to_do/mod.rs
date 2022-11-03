// This module is used to deal with passing parameters into view
mod create;
mod edit;
mod get;
mod utils;

use super::path::Path;
use actix_web::web;

pub fn item_factory(cfg: &mut web::ServiceConfig) {
    let base_path: Path = Path {
        prefix: String::from("/item"),
    };

    cfg.route(
        &base_path.define("/create/{title}".to_string()),
        web::post().to(create::create),
    )
    .route(
        &base_path.define("/get".to_string()),
        web::get().to(get::get),
    )
    .route(
        &base_path.define("/edit".to_string()),
        web::put().to(edit::edit),
    );
}
