#![feature(proc_macro_hygiene, decl_macro)]

use rocket_contrib::json::Json;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate rocket;

use mytodo::db::models::Task;
use mytodo::db::{query_task, establish_connection};

#[derive(Serialize)]
struct JsonApiResponse {
    data: Vec<Task>,
}

#[get("/tasks")]
fn tasks_get() -> Json<JsonApiResponse> {
    let mut response: JsonApiResponse = JsonApiResponse { data: vec![], };

    let conn = &mut establish_connection();
    query_task(conn).into_iter().for_each(|task| {
        response.data.push(task);
    });

    Json(response)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![tasks_get])
        .launch();
}

