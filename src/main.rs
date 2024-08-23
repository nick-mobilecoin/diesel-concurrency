use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;

#[macro_use]
extern crate diesel;

fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn update_counter(conn: &mut PgConnection, value: i32) {
    diesel::update(simple_table::table)
        .set(counters::value.eq(value))
        .execute(conn)
        .expect("Error updating counter");
}

fn insert_user(conn: &PgConnection, user_name: &str) {
    diesel::insert_into(users)
        .values(name.eq(user_name))
        .execute(conn)
        .expect("Error inserting user");
}

fn main() {
    let conn = Arc::new(Mutex::new(establish_connection()));

    let conn1 = Arc::clone(&conn);
    let conn2 = Arc::clone(&conn);

    let handle1 = thread::spawn(move || {
        let conn = conn1.lock().unwrap();
        insert_user(&conn, "User1");
    });

    let handle2 = thread::spawn(move || {
        let conn = conn2.lock().unwrap();
        insert_user(&conn, "User2");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Users inserted.");
}