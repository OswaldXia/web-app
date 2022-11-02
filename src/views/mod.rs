// Defining own factory for each views module
use actix_web::web;
mod path;
mod auth;
use std::env;

// we can merely pass in the config struct through the factory to build all the routes for the auth. 
// In the future, we will also build other modules for views. Because of this, we need another factory that can orchestrate the multiple view factories.
pub fn views_factory(cfg: &mut web::ServiceConfig) {
    let args = env::args();
    let logout = args.last().unwrap();
    if logout == "cancel_logout" {
        println!("logout view isn't being configured");
        auth::auth_factory(cfg, false);
    } else {
        println!("logout view is being configured");
        auth::auth_factory(cfg, true);
    }

    // We can simply cut off the `auth` views by merely commenting out the following line.
    // auth::auth_factory(cfg);
}