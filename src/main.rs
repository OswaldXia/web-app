mod auth;
mod database;
mod json_serialization;
mod models;
mod processes;
mod schema;
mod state;
mod to_do;
mod views;

use actix_web::{dev::Service, App, HttpResponse, HttpServer};
use env_logger;
use futures::future::{ok, Either};
use log;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(|| {
        let app = App::new()
            // pass in the service request and the routing
            .wrap_fn(|req, srv| {
                // get the uri path in case of being destroyed
                let req_url = req.uri().path().to_string();
                // flag used to indicate whether the authentication is passed
                let passed: bool;
                if req.path().contains("/item/") && auth::process_token(&req).is_err() {
                    passed = false;
                } else {
                    // not checking credentials for other calls
                    passed = true;
                }

                let future_result = if passed {
                    Either::Left(srv.call(req))
                } else {
                    Either::Right(ok(req.into_response(HttpResponse::Unauthorized().finish())))
                };

                async move {
                    // await the future and get the status result of the call, so we now have access to the result before returning it
                    let result = future_result.await?;
                    log::info!("{} -> {}", req_url, result.status());
                    Ok(result)
                }
            })
            .configure(views::views_factory);
        return app;
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
