/*
@generated automatically by Diesel CLI.
*/

table! {
    task (id) {
        id -> Integer,
        title -> Text,
        done -> Bool,
    }
}