mod auth;
mod database;
mod json_serialization;
mod models;
mod processes;
mod schema;
mod state;
mod to_do;
mod views;

use actix_web::dev::Service;
use actix_web::{App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let app = App::new()
            // pass in the service request and the routing
            .wrap_fn(|req, srv| {
                if req.path().contains("/item/") {
                    match auth::process_token(&req) {
                        Ok(_) => println!("the token is passable"),
                        Err(msg) => println!("token error: {}", msg),
                    }
                }
                // create a future with the call function belonging to the routing
                let fut = srv.call(req);
                async {
                    // await for this to complete and return the result
                    let result = fut.await?;
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
