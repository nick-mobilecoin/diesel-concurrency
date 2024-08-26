use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel_concurrency::schema::{
    foreign_key_column_table, serial_key_table, unique_column_table, unique_string_column_table,
    uuid_key_table,
};
use diesel_concurrency::{establish_connection, run_migrations};
use rand::random;
use std::thread;
use std::thread::sleep;
use uuid::Uuid;

fn insert_serial_key_value(conn: &mut PgConnection, value: i32) -> QueryResult<usize> {
    diesel::insert_into(serial_key_table::table)
        .values((
            serial_key_table::name.eq("hello"),
            serial_key_table::some_value.eq(value),
        ))
        .execute(conn)
}

fn insert_uuid_key_value(conn: &mut PgConnection, value: i32) -> QueryResult<usize> {
    diesel::insert_into(uuid_key_table::table)
        .values((
            uuid_key_table::name.eq("hello"),
            uuid_key_table::some_value.eq(value),
        ))
        .execute(conn)
}

fn insert_unique_column(conn: &mut PgConnection, value: i32) -> QueryResult<usize> {
    diesel::insert_into(unique_column_table::table)
        .values((
            unique_column_table::name.eq("hello"),
            unique_column_table::some_value.eq(value),
        ))
        .execute(conn)
}

fn insert_unique_string_column(conn: &mut PgConnection, value: i32) -> QueryResult<usize> {
    let value = value.to_string();
    diesel::insert_into(unique_string_column_table::table)
        .values((
            unique_string_column_table::name.eq("hello"),
            unique_string_column_table::some_value.eq(value),
        ))
        .execute(conn)
}

fn insert_foreign_key_column(conn: &mut PgConnection, value: i32) -> QueryResult<usize> {
    let id: Uuid = diesel::insert_into(uuid_key_table::table)
        .values((
            uuid_key_table::name.eq("hello"),
            uuid_key_table::some_value.eq(value),
        ))
        .returning(uuid_key_table::id)
        .get_result(conn)?;
    sleep(std::time::Duration::from_millis(random::<u64>() % 50));
    diesel::insert_into(foreign_key_column_table::table)
        .values((
            foreign_key_column_table::name.eq("hello"),
            foreign_key_column_table::uuid_id.eq(id),
        ))
        .execute(conn)
}

fn run(thread_name: &str, start_value: i32, stop_flag: Arc<AtomicBool>) {
    let mut connection = establish_connection();
    let mut offset = 0;
    loop {
        let result = connection.build_transaction().serializable().run(|conn| {
            sleep(std::time::Duration::from_millis(random::<u64>() % 50));
            // insert_serial_key_value(conn, start_value + offset)
            // insert_uuid_key_value(conn, start_value + offset)
            // insert_unique_column(conn, start_value + offset)
            // insert_unique_string_column(conn, start_value + offset)
            insert_foreign_key_column(conn, start_value + offset)
        });
        println!("Result for {thread_name}: {:?}", result);
        // These take a time to conflict so sometimes it's best
        // to let it run and then explode
        match result {
            Ok(_) => {}
            Err(_) => {
                stop_flag.store(true, std::sync::atomic::Ordering::Relaxed);
            }
        }
        offset += 1;
        if(stop_flag.load(std::sync::atomic::Ordering::Relaxed)) {
            break;
        }
    }
}

fn main() {
    run_migrations();

    {
        let mut conn = establish_connection();
        diesel::delete(serial_key_table::table)
            .execute(&mut conn)
            .unwrap();
        diesel::delete(foreign_key_column_table::table)
            .execute(&mut conn)
            .unwrap();
        diesel::delete(uuid_key_table::table)
            .execute(&mut conn)
            .unwrap();
        diesel::delete(unique_column_table::table)
            .execute(&mut conn)
            .unwrap();
        diesel::delete(unique_string_column_table::table)
            .execute(&mut conn)
            .unwrap();
    }

    let stop_flag = Arc::new(AtomicBool::new(false));

    let stop_flag_clone = stop_flag.clone();
    let handle1 = thread::spawn(move || {
        run("Thread 1", 0, stop_flag_clone);
    });

    let handle2 = thread::spawn(move || {
        run("Thread 2", 1_000_000, stop_flag);
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
}
