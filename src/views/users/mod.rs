use super::path::Path;
use actix_web::web::{self, ServiceConfig};

pub mod create;

pub fn user_factory(cfg: &mut ServiceConfig) {
    let base_path = Path {
        prefix: "/user/".to_string(),
    };

    cfg.route(&base_path.define("create"), web::post().to(create::create));
}
