// @generated automatically by Diesel CLI.

diesel::table! {
    persons (id, valid_from) {
        id -> Int4,
        email -> Text,
        password_hash -> Text,
        name -> Text,
        valid_from -> Timestamp,
        valid_to -> Nullable<Timestamp>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
