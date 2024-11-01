use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use std::sync::Arc;

pub struct ApiKey(pub String);

#[derive(Clone)]
pub struct Database {
    pub pool: Arc<r2d2::Pool<ConnectionManager<SqliteConnection>>>,
}

impl Database {
    pub fn new(database_url: String) -> Self {
        let manager = ConnectionManager::<SqliteConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        Database {
            pool: Arc::new(pool),
        }
    }

    pub fn initialize(&self) {
        let mut conn = self.pool.get().expect("Failed to get connection");
        diesel::sql_query(
            "CREATE TABLE IF NOT EXISTS chat_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                session_id TEXT NOT NULL,
                question TEXT NOT NULL,
                answer TEXT NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
            )",
        )
        .execute(&mut conn)
        .expect("Failed to create table");
    }

    pub fn checkpoint(&self) -> Result<(), String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        diesel::sql_query("PRAGMA wal_checkpoint(TRUNCATE)")
            .execute(&mut conn)
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
