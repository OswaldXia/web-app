// procedure to process a security JSON web token
use actix_web::dev::ServiceRequest;

// password checking
fn check_password(password: String) -> Result<String, &'static str> {
    if password == "token" {
        Ok(password)
    } else {
        Err("token not authorised")
    }
}

// password extraction
fn extract_header_token(request: &ServiceRequest) -> Result<String, &'static str> {
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

// public function to act as an entry point to this process and orchestrate these above functions
// The benefit of this approach is that if we want to add extra functionality, we can slot it in and out of the process.
pub fn process_token(request: &ServiceRequest) -> Result<String, &'static str> {
    // matching the result of the extraction of the password, and then matching the password check.
    match extract_header_token(request) {
        Ok(password) => check_password(password),
        Err(msg) => Err(msg),
    }
}
