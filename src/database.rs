// In this section, we will connect our application to the database running in Docker. In order to connect, we have to build a function that establishes a connection, and then returns it.
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    // to ensure that the program will not throw an error if we fail to load the environment
    dotenv().ok();
    // get our database URL from the environment variables and establish a connection using a reference to our database URL
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
