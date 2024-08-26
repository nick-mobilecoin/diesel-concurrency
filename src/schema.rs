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
    uuid_key_table (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        some_value -> Int4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    concurrent_update_table,
    serial_key_table,
    unique_column_table,
    uuid_key_table,
);
