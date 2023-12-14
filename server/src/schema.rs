// @generated automatically by Diesel CLI.

diesel::table! {
    person_roles (person_id, person_valid_from, role_id, valid_from) {
        person_id -> Int4,
        person_valid_from -> Timestamp,
        role_id -> Int4,
        valid_from -> Timestamp,
        valid_to -> Timestamp,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    persons (id, valid_from) {
        id -> Int4,
        email -> Text,
        password_hash -> Nullable<Text>,
        name -> Text,
        locked -> Bool,
        valid_from -> Timestamp,
        valid_to -> Timestamp,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    privileges (id) {
        id -> Int4,
        name -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    role_privileges (role_id, privilege_id) {
        role_id -> Int4,
        privilege_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    roles (id) {
        id -> Int4,
        name -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    streams (id, valid_from) {
        id -> Int4,
        name -> Text,
        valid_from -> Timestamp,
        valid_to -> Timestamp,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(person_roles -> roles (role_id));
diesel::joinable!(role_privileges -> privileges (privilege_id));
diesel::joinable!(role_privileges -> roles (role_id));

diesel::allow_tables_to_appear_in_same_query!(
    person_roles,
    persons,
    privileges,
    role_privileges,
    roles,
    streams,
);
