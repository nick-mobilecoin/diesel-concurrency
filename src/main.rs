mod schema;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::env;
use std::thread;
use diesel::query_builder::UpdateStatement;
use schema::simple_table;
use crate::schema::simple_table::table;
use diesel::prelude::*;

fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn update_query(conn: &mut PgConnection, user_name: &str, value: i32) -> UpdateStatement<table, WhereClause<diesel::dsl::Eq<simple_table::name, &str>>, Assign<ColumnWrapperForUpdate<simple_table::counter>, Bound<Integer, i32>>> {
    diesel::update(simple_table::table)
        .filter(simple_table::name.eq(user_name))
        .set(simple_table::counter.eq(value))
}
fn update_counter(conn: &mut PgConnection, user_name: &str, value: i32) {
    let result = diesel::update(simple_table::table)
        .filter(simple_table::name.eq(user_name))
        .set(simple_table::counter.eq(value)).execute(conn);
    println!("Result: {:?}", result);
}

fn insert_user(conn: &mut PgConnection, user_name: &str) {
    diesel::insert_into(simple_table::table)
        .values(simple_table::name.eq(user_name))
        .execute(conn)
        .expect("Error inserting user");
}

fn run(user_name: &str) {
    let mut conn = establish_connection();
    insert_user(&mut conn, user_name);
    loop {
        update_counter(&mut conn, user_name, 1);
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