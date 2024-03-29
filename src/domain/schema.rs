// @generated automatically by Diesel CLI.

diesel::table! {
    account (id) {
        id -> Int4,
        name -> Varchar,
        created_date -> Timestamptz,
        deleted_date -> Nullable<Timestamptz>,
        is_deleted -> Bool,
    }
}

diesel::table! {
    application (id) {
        id -> Int4,
        ulid -> Varchar,
        name -> Varchar,
        contact_email -> Varchar,
        account_id -> Int4,
        created_date -> Timestamptz,
        deleted_date -> Nullable<Timestamptz>,
        is_deleted -> Bool,
    }
}

diesel::table! {
    cron_logs (id) {
        id -> Int4,
        command -> Varchar,
        command_args -> Varchar,
        exit_status -> Nullable<Int4>,
        exit_message -> Nullable<Text>,
        started_at -> Timestamptz,
        ended_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    refresh_token (id) {
        id -> Int4,
        token -> Varchar,
        user_id -> Int4,
        validity_date -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        email -> Nullable<Varchar>,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        login -> Varchar,
        roles -> Array<Text>,
        password -> Text,
        salt -> Nullable<Text>,
        application_id -> Int4,
        created_date -> Timestamptz,
        created_by -> Nullable<Int4>,
        deleted_date -> Nullable<Timestamptz>,
        deleted_by -> Nullable<Int4>,
        is_deleted -> Bool,
    }
}

diesel::joinable!(application -> account (account_id));
diesel::joinable!(refresh_token -> users (user_id));
diesel::joinable!(users -> application (application_id));

diesel::allow_tables_to_appear_in_same_query!(
    account,
    application,
    cron_logs,
    refresh_token,
    users,
);
