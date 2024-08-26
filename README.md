An example of database contention on the same table

## Getting started
1. Clone the repository
2. create a database and set the `DATABASE_URL`
    ```bash
    createdb diesel_concurrency
    export DATABASE_URL=postgres://localhost/diesel_concurrency
    ```
3. Run the desired binary:
   1. Concurrently updating rows in the same table
       ```bash
       cargo run --bin update
       ```
   2. Modify the `serializable()` transaction in `update.rs` as needed
       ```rust
       let result = connection.build_transaction().serializable().run(|conn| {
           // lock_row(conn, user_name)?;
           // update_counter_raw(conn, user_name, counter)
           lock_table(conn)?;
           update_counter(conn, user_name, counter)
       });
   1. Concurrently inserting rows in the same table
       ```bash
       cargo run --bin insert
       ```
   2. Modify the `serializable()` transaction in `insert.rs` as needed
       ```rust
       let result = connection.build_transaction().serializable().run(|conn| {
           sleep(std::time::Duration::from_millis(random::<u64>() % 50));
           // insert_serial_key_value(conn, start_value + offset)
           // insert_uuid_key_value(conn, start_value + offset)
           // insert_unique_column(conn, start_value + offset)
           // insert_unique_string_column(conn, start_value + offset)
           insert_foreign_key_column(conn, start_value + offset)
       });
