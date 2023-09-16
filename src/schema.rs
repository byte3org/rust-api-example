// @generated automatically by Diesel CLI.

diesel::table! {
    catalog (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
        image -> Varchar,
        currency -> Varchar,
        category -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    category (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    catalog,
    category,
);
