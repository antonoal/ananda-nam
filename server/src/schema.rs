// @generated automatically by Diesel CLI.

diesel::table! {
    group_persons (person_id, group_id, valid_from) {
        person_id -> Int4,
        group_id -> Int4,
        group_role_id -> Nullable<Int4>,
        valid_from -> Timestamp,
        valid_to -> Nullable<Timestamp>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    group_roles (id) {
        id -> Int4,
        name -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    groups (id, valid_from) {
        id -> Int4,
        parent_id -> Nullable<Int4>,
        year_id -> Nullable<Int4>,
        stream_id -> Int4,
        name -> Text,
        valid_from -> Timestamp,
        valid_to -> Timestamp,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    person_roles (person_id, role_id, valid_from) {
        person_id -> Int4,
        role_id -> Int4,
        school_id -> Nullable<Int4>,
        year_id -> Nullable<Int4>,
        stream_id -> Nullable<Int4>,
        group_id -> Nullable<Int4>,
        entity_id -> Nullable<Int4>,
        entity_type -> Nullable<Text>,
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
    schools (id, valid_from) {
        id -> Int4,
        name -> Text,
        valid_from -> Timestamp,
        valid_to -> Timestamp,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    streams (id, valid_from) {
        id -> Int4,
        school_id -> Int4,
        name -> Text,
        start_year -> Int2,
        valid_from -> Timestamp,
        valid_to -> Timestamp,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    years (id, valid_from) {
        id -> Int4,
        school_id -> Int4,
        name -> Text,
        valid_from -> Timestamp,
        valid_to -> Timestamp,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(group_persons -> group_roles (group_role_id));
diesel::joinable!(person_roles -> roles (role_id));
diesel::joinable!(role_privileges -> privileges (privilege_id));
diesel::joinable!(role_privileges -> roles (role_id));

diesel::allow_tables_to_appear_in_same_query!(
    group_persons,
    group_roles,
    groups,
    person_roles,
    persons,
    privileges,
    role_privileges,
    roles,
    schools,
    streams,
    years,
);
