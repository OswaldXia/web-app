use actix_web::HttpResponse;
use super::content_loader::read_file;


pub async fn login() -> HttpResponse {
    // read data
    let mut login_html = read_file("./templates/login.html");
    let login_js = &read_file("./javascript/login.js");
    let main_css = &read_file("./css/main.css");
    let base_css = &read_file("./css/base.css");

    login_html = login_html
        .replace("{ JAVASCRIPT }", login_js)
        .replace("/* CSS */", main_css)
        .replace("/* BASE_CSS */", base_css);

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(login_html)
}

