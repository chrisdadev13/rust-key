use self::models::*;
use diesel::prelude::*;
use key::*;

fn main() {
    use self::schema::credentials::dsl::*;

    let connection = &mut establish_connection();
    let results = credentials
        .select(Credentials::as_select())
        .load(connection)
        .expect("Error loading credentials");

    println!("It worked? {}", results.len());
}
