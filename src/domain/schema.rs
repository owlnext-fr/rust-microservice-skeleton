// @generated automatically by Diesel CLI.

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
        created_date -> Timestamptz,
        created_by -> Nullable<Int4>,
        deleted_date -> Nullable<Timestamptz>,
        deleted_by -> Nullable<Int4>,
        is_deleted -> Bool,
    }
}

diesel::joinable!(refresh_token -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(cron_logs, refresh_token, users,);
