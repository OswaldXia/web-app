pub mod jwt;
mod token;

use actix_web::dev::ServiceRequest;

// public function to act as an entry point to this process and orchestrate these above functions
// The benefit of this approach is that if we want to add extra functionality, we can slot it in and out of the process.
pub fn process_token(request: &ServiceRequest) -> Result<&str, &'static str> {
    // matching the result of the extraction of the password, and then matching the password check.
    match token::extract_header_token(request) {
        Ok(password) => token::check_password(password),
        Err(msg) => Err(msg),
    }
}
