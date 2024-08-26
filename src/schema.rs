// @generated automatically by Diesel CLI.

diesel::table! {
    concurrent_update_table (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        counter -> Int4,
    }
}

diesel::table! {
    foreign_key_column_table (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        uuid_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    serial_key_table (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        some_value -> Int4,
    }
}

diesel::table! {
    unique_column_table (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        some_value -> Int4,
    }
}

diesel::table! {
    unique_string_column_table (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        some_value -> Varchar,
    }
}

diesel::table! {
    uuid_key_table (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        some_value -> Int4,
    }
}

diesel::joinable!(foreign_key_column_table -> uuid_key_table (uuid_id));

diesel::allow_tables_to_appear_in_same_query!(
    concurrent_update_table,
    foreign_key_column_table,
    serial_key_table,
    unique_column_table,
    unique_string_column_table,
    uuid_key_table,
);
