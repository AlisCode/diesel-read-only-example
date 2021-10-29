use crate::schema::users;
use diesel::backend::ReadOnly;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

#[macro_use]
extern crate diesel;

mod schema;

fn main() {
    //let conn = SqliteConnection::establish("test.db").expect("Failed to connect");
    //create_user(&conn, "Olivier");
    let ro_conn = ReadOnly::<SqliteConnection>::establish("test.db").expect("Failed to connect");
    read_users(&ro_conn);
}

fn create_user(conn: &SqliteConnection, name: &str) {
    diesel::insert_into(users::table)
        .values(User {
            id: 1,
            name: name.to_string(),
        })
        .execute(conn)
        .expect("Failed to insert");
}

fn read_users(conn: &ReadOnly<SqliteConnection>) {}

#[derive(Debug, Queryable, Insertable)]
#[table_name = "users"]
struct User {
    id: i32,
    name: String,
}
