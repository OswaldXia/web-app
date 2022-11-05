use actix_web::HttpResponse;

use super::content_loader::read_file;

pub async fn items() -> HttpResponse {
    let html_file_path = "./templates/main.html";
    let mut html_data = read_file(html_file_path);
    let js_file_path = "./javascript/main.js";
    let js_code = read_file(js_file_path);

    let css_data: String = read_file("./css/main.css");
    let base_css_data: String = read_file("./css/base.css");

    html_data = html_data
        .replace("{ JAVASCRIPT }", &js_code)
        .replace("/* BASE_CSS */", &base_css_data)
        .replace("/* CSS */", &css_data);
    // simply return a HttpResponse struct that has a HTML content type and a body
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html_data)
}
