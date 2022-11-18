// procedure to process a security JSON web token
use super::jwt::JwtToken;
use actix_web::dev::ServiceRequest;

// password checking
pub fn check_password(password: &str) -> Result<&str, &'static str> {
    match JwtToken::decode(password) {
        Ok(_) => Ok("passed"),
        Err(msg) => Err(msg),
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

#[cfg(test)]
mod credentials_tests {
    use super::*;
    use crate::auth::jwt::JwtToken;
    use actix_web::test::TestRequest;
    // use actix_web::test;

    #[test]
    fn correct_check_password() {
        let password = &JwtToken::encode(32);
        let result = check_password(password);

        if let Ok(msg) = result {
            assert_eq!(msg, "passed");
        } else {
            panic!("correct password should pass");
        }
    }

    #[test]
    fn incorrect_check_password() {
        let password = "test";
        let result = check_password(password);

        assert_eq!(
            result,
            Err("Could not decode"),
            "incorrect password should not be decoded"
        );
    }

    #[test]
    fn no_token_in_extract_header_token() {
        let mock_request = &TestRequest::default()
            .insert_header(("test", "test"))
            .to_srv_request();
        
        assert_eq!(
            extract_header_token(mock_request),
            Err("There is no token"),
            "token should not be present in service request"
        )
    }

    #[test]
    fn correct_token_in_extract_header_token() {
        let mock_request = &TestRequest::default()
            .insert_header(("user-token", "test"))
            .to_srv_request();
        
        assert_eq!(
            extract_header_token(mock_request),
            Ok("test"),
            "token should be present in the header"
        );
    }
}
