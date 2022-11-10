// procedure to process a security JSON web token
use actix_web::dev::ServiceRequest;

// password checking
pub fn check_password(password: String) -> Result<String, &'static str> {
    if password == "token" {
        Ok(password)
    } else {
        Err("token not authorised")
    }
}

// password extraction
pub fn extract_header_token(request: &ServiceRequest) -> Result<String, &'static str> {
    if let Some(token) = request.headers().get("user-token") {
        if let Ok(password) = token.to_str() {
            Ok(password.to_string())
        } else {
            Err("There is an error processing token")
        }
    } else {
        Err("There is no token")
    }
}


