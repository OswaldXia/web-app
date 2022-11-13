use actix_web::HttpResponse;

use super::content_loader::read_file;

//  remove the user token from the local storage, and then revert the user back to the main view
pub async fn logout() -> HttpResponse {
    let logout_html = read_file("./templates/logout.html");

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(logout_html)
}

