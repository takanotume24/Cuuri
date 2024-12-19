use diesel::prelude::*;
use std::result::Result;

pub fn establish_connection(database_path: &str) -> Result<SqliteConnection, diesel::ConnectionError> {
    SqliteConnection::establish(database_path)
}
