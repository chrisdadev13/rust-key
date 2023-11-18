// @generated automatically by Diesel CLI.

diesel::table! {
    credentials (id) {
        id -> Integer,
        url -> Nullable<Text>,
        account_name -> Text,
        password -> Text,
        category -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}
