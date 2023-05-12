use diesel::{prelude::*, sqlite::SqliteConnection};

use self::schema::task::{
    self,
    dsl::{done, id},
};
use exitcode;
pub use pad::{Alignment, PadStr};
use std::{
    io::BufRead,
    process,
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
    Get the vector from QueryResult.
    */
    if let Ok(binding) = my_data {
        /*
        Get the value from the vector element.
        Based on the SQL statement, there
        should only be one value.
        */
        let y = &binding[0].title;
        /*
        Whenever you parse without an unwrap(),
        you get a Result value.  So, if the
        conversion is ok, return the value.
        */
        if let Ok(y) = y.parse::<i32>() {
            return y;
        } else {
            /*
            Conversion error occured.
            */
            return 0;
        }
    } else {
        /*
        Error occured.
        */
        return 0;
    }
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
        from an Option.
        */
        .unwrap()
        /*
        This unwrap() returns a String
        from a Result
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

pub fn display_header() -> usize {
    let nbr = max_title() as usize;
    let pending_task: &str = "<PendingTask>";

    match nbr > 0 {
        true => {
            let _id = "ID".pad_to_width_with_alignment(6, Alignment::Left);
            let _title = "TITLE".pad_to_width_with_alignment(nbr + 3, Alignment::Left);
            let _done = "DONE".pad_to_width_with_alignment(nbr + 3, Alignment::Left);

            let id_ = "-"
                .repeat(3)
                .pad_to_width_with_alignment(6, Alignment::Left);
            let title_ = "-"
                .repeat(nbr)
                .pad_to_width_with_alignment(nbr + 3, Alignment::Left);
            let done_ = "-"
                .repeat(pending_task.len())
                .pad_to_width_with_alignment(nbr + 3, Alignment::Left);

            print!("\n{}{}{}\n{}{}{}\n", _id, _title, _done, id_, title_, done_);
        }
        false => {
            print!("");
        }
    }
    nbr
}
