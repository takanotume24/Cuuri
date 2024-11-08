use crate::database::Database;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::PooledConnection;
use diesel::sqlite::SqliteConnection;
use tauri::State;

pub fn get_db_connection(
    db: &State<'_, Database>,
) -> Result<PooledConnection<ConnectionManager<SqliteConnection>>, String> {
    db.pool.get().map_err(|e| e.to_string())
}
