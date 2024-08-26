use std::thread;
use diesel::{PgConnection, QueryResult, RunQueryDsl};
use diesel_concurrency::{establish_connection, run_migrations};
use diesel_concurrency::schema::serial_key_table;

fn run(thread_name: &str) {
    let mut connection = establish_connection();
    let mut value = 0;
    loop {
        let result = connection.build_transaction().serializable().run(|conn| {
            insert_serial_key_value(conn, value)
        });
        println!("Result for {thread_name}: {:?}", result);
        value += 1;
    }
}

fn insert_serial_key_value(conn: &mut PgConnection, value: i32) -> QueryResult<usize> {
    diesel::insert_into(serial_key_table::table)
        .values((
            serial_key_table::some_value.eq(value),
        ))
        .execute(conn)
        .expect("Error inserting value")
}

fn main() {
    run_migrations();

    let handle1 = thread::spawn(move || {
        run("User1");
    });

    let handle2 = thread::spawn(move || {
        run("User2");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}