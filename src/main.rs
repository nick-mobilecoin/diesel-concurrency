mod schema;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::env;
use std::thread;
use schema::simple_table;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn insert_user(conn: &mut PgConnection, user_name: &str) {
    diesel::insert_into(simple_table::table)
        .values(simple_table::name.eq(user_name))
        .on_conflict_do_nothing()
        .execute(conn)
        .expect("Error inserting user");
}

fn update_counter(conn: &mut PgConnection, user_name: &str, value: i32) -> QueryResult<usize> {
    diesel::update(simple_table::table)
        .filter(simple_table::name.eq(user_name))
        .set(simple_table::counter.eq(value))
        .execute(conn)
}

fn update_counter_raw(conn: &mut PgConnection, user_name: &str, value: i32) -> QueryResult<usize> {
    diesel::sql_query("UPDATE simple_table SET counter = $1 WHERE name = $2")
        .bind::<diesel::sql_types::Int4, _>(value)
        .bind::<diesel::sql_types::VarChar, _>(user_name)
        .execute(conn)
}

fn lock_row(conn: &mut PgConnection, user_name: &str) -> QueryResult<()> {
    diesel::sql_query("SELECT * FROM simple_table WHERE name = $1 FOR UPDATE")
        .bind::<diesel::sql_types::VarChar, _>(user_name)
        .execute(conn)
        .map(|_| ())
}

fn for_update(conn: &mut PgConnection) {
    simple_table::table.for_update().execute(conn).expect("Error locking table");
}

fn run(user_name: &str) {
    let mut connection = establish_connection();
    insert_user(&mut connection, user_name);
    let mut counter = 0;
    loop {
        let result = connection.build_transaction().serializable().run(|conn| {
            // lock_row(conn, user_name)?;
            // update_counter_raw(conn, user_name, counter)
            for_update(conn);
            update_counter(conn, user_name, counter)
        });
        println!("Result for {user_name}: {:?}", result);
        counter += 1;
    }
}

fn main() {
    {
        let mut connection = establish_connection();
        connection.run_pending_migrations(MIGRATIONS).expect("Failed to run migrations");
    }

    let handle1 = thread::spawn(move || {
        run("User1");
    });

    let handle2 = thread::spawn(move || {
        run("User2");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}