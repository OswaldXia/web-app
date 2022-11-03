use actix_web::web;
mod login;
mod logout;
use super::path::Path;

pub fn auth_factory(cfg: &mut web::ServiceConfig) {
    let base_path = Path {
        prefix: "/auth".to_string(),
    };

    cfg.route(
        &base_path.define("/login".to_string()),
        web::get().to(login::login),
    )
    .route(
        &base_path.define(String::from("/logout")),
        web::get().to(logout::logout),
    );
}
