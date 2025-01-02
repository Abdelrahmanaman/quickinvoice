// @generated automatically by Diesel CLI.

diesel::table! {
    invoices (id) {
        id -> Uuid,
        user_id -> Uuid,
        amount -> Numeric,
        currency -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
        api_key -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    webhooks (id) {
        id -> Uuid,
        user_id -> Uuid,
        event_type -> Text,
        payload -> Jsonb,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    invoices,
    users,
    webhooks,
);
