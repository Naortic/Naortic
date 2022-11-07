// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Integer,
        name -> Varchar,
        password -> Varchar,
        token -> Varchar,
        email -> Varchar,
    }
}
