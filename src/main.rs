mod auth;
mod database;
mod json_serialization;
mod models;
mod processes;
mod schema;
mod state;
mod to_do;
mod views;

use actix_web::{
    dev::Service,
    App, HttpResponse, HttpServer,
};

use futures::future::{ok, Either};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let app = App::new()
            // pass in the service request and the routing
            .wrap_fn(|req, srv| {
                // flag used to indicate whether the authentication is passed
                let passed: bool;
                if req.path().contains("/item/") 
                && auth::process_token(&req).is_err() {
                    passed = false;
                } else {
                    // not checking credentials for other calls
                    passed = true;
                }

                if passed {
                    Either::Left(srv.call(req))
                } else {
                    Either::Right(ok(req.into_response(HttpResponse::Unauthorized().finish())))
                }
            })
            .configure(views::views_factory);
        return app;
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
