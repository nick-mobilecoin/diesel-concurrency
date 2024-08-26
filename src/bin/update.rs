use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::env;
use std::thread;
use diesel_concurrency::schema::concurrent_update_table;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn insert_user(conn: &mut PgConnection, user_name: &str) {
    diesel::insert_into(concurrent_update_table::table)
        .values(concurrent_update_table::name.eq(user_name))
        .on_conflict_do_nothing()
        .execute(conn)
        .expect("Error inserting user");
}

fn update_counter(conn: &mut PgConnection, user_name: &str, value: i32) -> QueryResult<usize> {
    diesel::update(concurrent_update_table::table)
        .filter(concurrent_update_table::name.eq(user_name))
        .set(concurrent_update_table::counter.eq(value))
        .execute(conn)
}

// A raw SQL query just to be sure the `update_counter` above is not the issue
fn update_counter_raw(conn: &mut PgConnection, user_name: &str, value: i32) -> QueryResult<usize> {
    diesel::sql_query("UPDATE concurrent_update_table SET counter = $1 WHERE name = $2")
        .bind::<diesel::sql_types::Int4, _>(value)
        .bind::<diesel::sql_types::VarChar, _>(user_name)
        .execute(conn)
}

// For the life of me I can't figure out how to get the table name from the
// table struct, but doing a table lock seems to prevent serialization errors
// when updating the same table
fn lock_table(conn: &mut PgConnection) -> QueryResult<()> {
    diesel::sql_query("LOCK TABLE concurrent_update_table IN EXCLUSIVE MODE")
        .execute(conn)
        .map(|_| ())
}

// One can look at https://www.postgresql.org/docs/current/explicit-locking.html#LOCKING-ROWS
// it mentions
// > Within a REPEATABLE READ or SERIALIZABLE transaction, however, an error
// > will be thrown if a row to be locked has changed since the transaction started
// My interpretation of this is that if one uses `for_update` serialized transactions
// will still throw an error when hitting a for update lock
fn for_update(conn: &mut PgConnection) {
    concurrent_update_table::table.for_update().execute(conn).expect("Error locking table");
}

// See above for "FOR UPDATE"
fn lock_row(conn: &mut PgConnection, user_name: &str) -> QueryResult<()> {
    diesel::sql_query("SELECT * FROM concurrent_update_table WHERE name = $1 FOR UPDATE")
        .bind::<diesel::sql_types::VarChar, _>(user_name)
        .execute(conn)
        .map(|_| ())
}

fn run(user_name: &str) {
    let mut connection = establish_connection();
    insert_user(&mut connection, user_name);
    let mut counter = 0;
    loop {
        let result = connection.build_transaction().serializable().run(|conn| {
            // lock_row(conn, user_name)?;
            // update_counter_raw(conn, user_name, counter)
            lock_table(conn)?;
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