// @generated automatically by Diesel CLI.

diesel::table! {
    notes (id) {
        id -> Nullable<Integer>,
        user_id -> Integer,
        content -> Nullable<Text>,
        created_at -> Integer,
        updated_at -> Integer,
    }
}

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        email -> Text,
        display_name -> Text,
        created_at -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(notes, users,);
