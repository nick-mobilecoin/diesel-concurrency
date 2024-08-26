// @generated automatically by Diesel CLI.

diesel::table! {
    update_row (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        counter -> Int4,
    }
}
