use diesel::{prelude::*, sqlite::SqliteConnection};

use self::schema::task::{
    self,
    dsl::{done, id},
};
use exitcode;
use std::{
    io::BufRead,
    process::{self},
};

pub mod models;
pub mod schema;

pub fn establish_connection() -> SqliteConnection {
    let db = "./testdb.sqlite3";
    SqliteConnection::establish(db).unwrap_or_else(|_| panic!("Error connecting to {}", db))
}

pub fn create_task(connection: &mut SqliteConnection, title: &str) {
    let task = models::NewTask { title };

    diesel::insert_into(schema::task::table)
        .values(&task)
        .execute(connection)
        .expect("Error inserting new task");
}

pub fn query_task(connection: &mut SqliteConnection) -> Vec<models::Task> {
    schema::task::table
        .filter(done.eq(false))
        .load::<models::Task>(connection)
        .expect("Error loading non-pending tasks")
}

pub fn query_display_task(connection: &mut SqliteConnection) -> Vec<models::Task> {
    schema::task::table
        .load::<models::Task>(connection)
        .expect("Error loading pending & non-pending tasks")
}

pub fn update_task(
    ids: Vec<i32>,
    connection: &mut SqliteConnection,
) -> Result<usize, diesel::result::Error> {
    diesel::update(schema::task::table)
        .set(task::done.eq(true))
        .filter(task::done.eq(false))
        .filter(id.eq_any(&ids))
        .execute(connection)
}

pub fn delete_task(
    ids: Vec<i32>,
    connection: &mut SqliteConnection,
) -> Result<usize, diesel::result::Error> {
    diesel::delete(schema::task::table)
        .filter(id.eq_any(&ids))
        .execute(connection)
}

pub fn max_title() -> i32 {
    use diesel::sql_query;

    let conn: &mut SqliteConnection = &mut establish_connection();

    let my_data: QueryResult<Vec<models::QueryableTask>> =
        sql_query("Select max(length(title)) as title from task")
            .load::<models::QueryableTask>(conn);

    /*
    Remove the Vec
    */
    let binding = my_data.unwrap();
    /*
    Get the value
    */
    let y = binding.get(0).unwrap();
    /*
    Convert the value to i32
    */
    let y = y.title.parse::<i32>().unwrap();

    /*
    Return value
    */
    return y;
}

pub fn read_input<T>() -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    std::io::stdin()
        /*
        Locks this handle to the standard input stream, returning a readable guard.
        The lock is released when the returned lock goes out of scope.
        The returned guard also implements the Read and BufRead
        traits for accessing the underlying data.
        */
        .lock()
        .lines()
        .next()
        /*
        This unwrap() returns Result<String, Error>
        */
        .unwrap()
        /*
        This unwrap() returns a String
        */
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| -> T {
            s.parse::<T>().unwrap_or_else(|e| {
                eprintln!("Could not parse input '{s}': {:?}", e);
                /*
                Shut down in a non-panicky manner due to
                user's input data being incorrect in some way.
                */
                process::exit(exitcode::DATAERR);
            })
        })
        .collect::<Vec<T>>()
}
