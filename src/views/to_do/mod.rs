// This module is used to deal with passing parameters into view
pub mod create;

use actix_web::web;
use super::path::Path;


pub fn item_factory(cfg: &mut web::ServiceConfig) {
    let base_path: Path = Path{prefix: String::from("/item")};

    cfg.route(&base_path.define("/create/{title}".to_string()), web::post().to(create::create));
}