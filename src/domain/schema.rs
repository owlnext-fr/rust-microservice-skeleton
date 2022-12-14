// @generated automatically by Diesel CLI.

diesel::table! {
    user (id) {
        id -> Int4,
        email -> Nullable<Varchar>,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        login -> Varchar,
        roles -> Array<Text>,
        password -> Varchar,
        salt -> Nullable<Text>,
        created_date -> Timestamptz,
        created_by -> Nullable<Int4>,
        deleted_date -> Nullable<Timestamptz>,
        deleted_by -> Nullable<Int4>,
        is_deleted -> Bool,
    }
}
