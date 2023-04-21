use super::schema::task;

#[derive(Insertable)]
#[diesel(table_name = task)]
pub struct NewTask<'a> {
    pub title: &'a str,
}

#[derive(Queryable, Debug)]
#[diesel(table_name = task)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub done: bool,
}

#[derive(QueryableByName, Debug)]
#[diesel(table_name = task)]
pub struct QueryableTask {
    pub title: String,
}
