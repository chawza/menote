// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        email -> Text,
        display_name -> Text,
        created_at -> Integer,
    }
}
