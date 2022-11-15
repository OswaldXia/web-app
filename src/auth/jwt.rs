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
