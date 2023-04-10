use diesel::{prelude::*, sqlite::SqliteConnection};
use self::{schema::task::{dsl::{done, id}, self}}; // models::Task};
//crate::db::schema::task::dsl::task;
pub mod models;
pub mod schema;


pub fn establish_connection() -> SqliteConnection {
    let db = "./testdb.sqlite3";
    SqliteConnection::establish(db)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db))
}

pub fn create_task(connection: &SqliteConnection, title: &str) {
    let task = models::NewTask { title };

    diesel::insert_into(schema::task::table)
        .values(&task)
        .execute(connection)
        .expect("Error inserting new task");
}


pub fn query_task(connection: &SqliteConnection) -> Vec<models::Task> {
    schema::task::table
        .filter(done.eq(false))
        .load::<models::Task>(connection)
        .expect("Error loading non-pending tasks")
}

pub fn query_display_task(connection: &SqliteConnection) -> Vec<models::Task> {
    schema::task::table
        .load::<models::Task>(connection)
        .expect("Error loading pending & non-pending tasks")
}

pub fn update_task(ids: Vec<i32>, connection: &SqliteConnection) -> Result<usize, diesel::result::Error> {
    let rtn = diesel::update(schema::task::table)
        .set(task::done.eq(true))
        .filter(task::done.eq(false))
        .filter(id.eq_any(&ids))
        .execute(connection);
    return rtn;
}


