An example of database contention on the same table

## Getting started
1. Clone the repository
2. create a database and set the `DATABASE_URL`
    ```bash
    createdb diesel_contention
    export DATABASE_URL=postgres://localhost/diesel_contention
    ```
3. Run the application
    ```bash
    cargo run
    ```
4. Modify the `serializable()` transaction in `main.rs` as needed
    ```rust
    let result = connection.build_transaction().serializable().run(|conn| {
        // lock_row(conn, user_name)?;
        // update_counter_raw(conn, user_name, counter)
        lock_table(conn)?;
        update_counter(conn, user_name, counter)
    });
    ```