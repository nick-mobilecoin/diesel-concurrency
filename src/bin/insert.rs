use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel_concurrency::schema::{serial_key_table, unique_column_table, uuid_key_table};
use diesel_concurrency::{establish_connection, run_migrations};
use std::thread;

fn run(thread_name: &str, start_value: i32) {
    let mut connection = establish_connection();
    let mut offset = 0;
    loop {
        let result = connection
            .build_transaction()
            .serializable()
            .run(|conn| insert_unique_column(conn, start_value + offset));
        println!("Result for {thread_name}: {:?}", result);
        offset += 1;
    }
}

fn insert_serial_key_value(conn: &mut PgConnection, value: i32) -> QueryResult<usize> {
    diesel::insert_into(serial_key_table::table)
        .values((serial_key_table::name.eq("hello"), serial_key_table::some_value.eq(value)))
        .execute(conn)
}

fn insert_uuid_key_value(conn: &mut PgConnection, value: i32) -> QueryResult<usize> {
    diesel::insert_into(uuid_key_table::table)
        .values((uuid_key_table::name.eq("hello"), uuid_key_table::some_value.eq(value)))
        .execute(conn)
}

fn insert_unique_column(conn: &mut PgConnection, value: i32) -> QueryResult<usize> {
    diesel::insert_into(unique_column_table::table)
        .values((unique_column_table::name.eq("hello"), unique_column_table::some_value.eq(value)))
        .execute(conn)
}

fn main() {
    run_migrations();

    let handle1 = thread::spawn(move || {
        run("Thread 1", 0);
    });

    let handle2 = thread::spawn(move || {
        run("Thread 2", 1_000_000);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}
