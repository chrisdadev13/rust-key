use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::credentials)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Credentials {
    pub id: i32,
    pub url: Option<String>,
    pub account_name: String,
    pub password: String,
    pub category: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::credentials)]
pub struct NewCredentials<'a> {
    pub url: Option<&'a str>,
    pub account_name: &'a str,
    pub password: &'a str,
    pub category: Option<&'a str>,
}
