use diesel::prelude::*;

pub fn establish_connection(database_path: &String) -> SqliteConnection {
    SqliteConnection::establish(&database_path)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_path))
}
