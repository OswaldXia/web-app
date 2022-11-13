// procedure to process a security JSON web token
use actix_web::dev::ServiceRequest;
use super::jwt::JwtToken;

// password checking
pub fn check_password(password: &str) -> Result<&str, &'static str> {
    match JwtToken::decode(password) {
        Ok(_) => Ok("passed"),
        Err(msg) => Err(msg)
    }
}

// password extraction
pub fn extract_header_token(request: &ServiceRequest) -> Result<&str, &'static str> {
    if let Some(token) = request.headers().get("user-token") {
        if let Ok(password) = token.to_str() {
            Ok(password)
        } else {
            Err("There is an error processing token")
        }
    } else {
        Err("There is no token")
    }
}


