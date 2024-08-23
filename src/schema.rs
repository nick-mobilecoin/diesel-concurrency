// @generated automatically by Diesel CLI.

diesel::table! {
    simple_table (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        counter -> Int4,
    }
}
