// define a login schema
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Login {
    pub user_name: String,
    pub password: String
}

