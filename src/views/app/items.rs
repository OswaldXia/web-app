use actix_web::HttpResponse;

use super::content_loader::read_file;

pub async fn items() -> HttpResponse {
    let file_path = "./templates/main.html";
    let html_data = read_file(file_path);
    // simply return a HttpResponse struct that has a HTML content type and a body
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html_data)
}
