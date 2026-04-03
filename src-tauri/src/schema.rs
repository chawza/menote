// @generated automatically by Diesel CLI.

diesel::table! {
    notes (id) {
        id -> Nullable<Integer>,
        user_id -> Integer,
        content -> Nullable<Text>,
        created_at -> BigInt,
        updated_at -> BigInt,
    }
}

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        email -> Text,
        display_name -> Text,
        created_at -> BigInt,
    }
}

diesel::allow_tables_to_appear_in_same_query!(notes, users,);
