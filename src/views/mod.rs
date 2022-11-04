// Defining own factory for each views module
mod auth;
mod path;
mod to_do;
pub mod token;
mod app;

use actix_web::web;

// we can merely pass in the config struct through the factory to build all the routes for the auth.
// In the future, we will also build other modules for views. Because of this, we need another factory that can orchestrate the multiple view factories.
pub fn views_factory(cfg: &mut web::ServiceConfig) {
    // We can simply cut off the `auth` views by merely commenting out the following line.
    auth::auth_factory(cfg);
    to_do::item_factory(cfg);
    app::app_factory(cfg);
}
