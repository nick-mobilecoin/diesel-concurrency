mod schema;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::env;
use std::thread;
use schema::simple_table;

fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn update_counter(conn: &mut PgConnection, user_name: &str, value: i32) -> QueryResult<usize> {
    diesel::update(simple_table::table)
        .filter(simple_table::name.eq(user_name))
        .set(simple_table::counter.eq(value)).execute(conn)
}

fn insert_user(conn: &mut PgConnection, user_name: &str) {
    diesel::insert_into(simple_table::table)
        .values(simple_table::name.eq(user_name))
        .execute(conn)
        .expect("Error inserting user");
}

fn run(user_name: &str) {
    let mut connection = establish_connection();
    insert_user(&mut connection, user_name);
    let mut counter = 0;
    loop {
        let result = connection.build_transaction().serializable().run(|conn| {
            update_counter(conn, user_name, counter)
        });
        println!("Result for {user_name}: {:?}", result);
        counter += 1;
    }
}

fn main() {
    let handle1 = thread::spawn(move || {
        run("User1");
    });

    let handle2 = thread::spawn(move || {
        run("User2");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}