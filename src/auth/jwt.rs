use actix_web::HttpRequest;
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use jwt::{Header, Token, VerifyWithKey};
use sha2::Sha256;
use std::collections::BTreeMap;

//  token struct
pub struct JwtToken {
    pub user_id: i32,
    pub body: String,
}

impl JwtToken {
    // pub fn encode() {}
    pub fn encode(user_id: i32) -> String {
        // created a new key with the secret byte string
        let key = Hmac::<Sha256>::new_from_slice(b"secret").unwrap();
        // created a new map and inserted the user ID
        let mut claims = BTreeMap::new();
        claims.insert("user_id", user_id);
        //  sign in with secret key and return the hashed token
        let token = claims.sign_with_key(&key).unwrap();
        token
    }

    pub fn decode(encoded_token: &str) -> Result<JwtToken, &'static str> {
        // generated a new key with the same byte string
        let key = &Hmac::<Sha256>::new_from_slice(b"secret").unwrap();
        // used VerifyWithKey with the token string and key we created to get the token
        let result = VerifyWithKey::<Token<Header, BTreeMap<String, _>, _>>::verify_with_key(
            encoded_token,
            key,
        );

        match result {
            Ok(decoded_token) => Ok(
                // generate struct with the user ID from claims map
                JwtToken {
                    // get map with the claims function
                    user_id: decoded_token.claims()["user_id"],
                    body: encoded_token.to_string(),
                },
            ),
            Err(_) => Err("Could not decode"),
        }
    }
    // accepts the HTTP request struct, extracts the token from the header, and then calls the decode function in order to avoid repetitive code in all our views that extract the header
    pub fn decode_from_request(request: &HttpRequest) -> Result<JwtToken, &'static str> {
        if let Some(token) = request.headers().get("user-token") {
            JwtToken::decode(token.to_str().unwrap())
        } else {
            Err("there is no token")
        }
    }
}

#[cfg(test)]
mod jwt_tests {
    use super::JwtToken;
    use actix_web::{test, test::TestRequest};

    // check to see if the whole process works if everything is right
    #[test]
    async fn encode_decode() {
        // encoded a random user ID number into a token
        let encoded_token = &JwtToken::encode(32);
        let decoded_token = JwtToken::decode(encoded_token).unwrap();
        assert_eq!(32, decoded_token.user_id);
    }

    // ensure that a failure in decoding the token will be handled correctly
    #[test]
    async fn decode_incorrect_token() {
        // passing a random string into the JwtToken::decode function.
        let encoded_token = "test";

        if let Err(msg) = JwtToken::decode(encoded_token) {
            assert_eq!("Could not decode", msg);
        } else {
            panic!("Incorrect token should not be able to be encoded");
        }
    }

    // testing our function that decodes from a request
    #[test]
    async fn decode_from_request_with_correct_token() {
        let encoded_token = JwtToken::encode(32);
        let request = &TestRequest::default()
            .insert_header(("user-token", encoded_token))
            .to_http_request();
        let result = JwtToken::decode_from_request(request);
        if let Ok(decoded_token) = result {
            assert_eq!(32, decoded_token.user_id)
        } else {
            panic!("Token is not returned when it should be")
        }
    }

    #[test]
    async fn decode_from_request_with_no_token() {
        let request = &TestRequest::default().to_http_request();
        let result = JwtToken::decode_from_request(request);

        if let Err(msg) = result {
            assert_eq!("there is no token", msg);
        } else {
            panic!("Token should not be returned when it is not present in the headers");
        }
    }

    #[test]
    async fn decode_from_request_with_false_token() {
        let request = &TestRequest::default()
            .insert_header(("user-token", "test"))
            .to_http_request();
        let result = JwtToken::decode_from_request(request);

        if let Err(msg) = result {
            assert_eq!("Could not decode", msg);
        } else {
            panic!("should be an error with a fake token");
        }
    }
}
