use bcrypt::{hash, DEFAULT_COST};
use diesel::Insertable;
use uuid::Uuid;
use crate::schema::users;

#[derive(Insertable, Clone)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub unique_id: String,
}

impl NewUser {
    pub fn new(username: &str, email: &str, password: &str) -> NewUser {
        let hashed_pwd = hash(password, DEFAULT_COST).unwrap();
        let uuid = Uuid::new_v4().to_string();
        NewUser {
            username: username.to_string(),
            email: email.to_string(),
            password: hashed_pwd,
            unique_id: uuid,
        }
    }
}
