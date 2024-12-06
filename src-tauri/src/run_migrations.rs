use diesel::prelude::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn run_migrations(connection: &mut SqliteConnection) {
    connection.run_pending_migrations(MIGRATIONS).unwrap();
}
