pub mod models;
pub mod schema;

use std::env;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;

use crate::models::NewCredentials;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to the database"))
}

pub fn create_credentials(
    conn: &mut SqliteConnection,
    url: Option<&str>,
    account_name: &str,
    password: &str,
    category: Option<&str>,
) {
    use crate::schema::credentials;

    let new_credential = NewCredentials {
        url,
        account_name,
        password,
        category,
    };

    diesel::insert_into(credentials::table)
        .values(&new_credential)
        .execute(conn)
        .expect("Something bad happened during creation");
}
