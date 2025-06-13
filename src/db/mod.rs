use diesel::{r2d2, SqliteConnection};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness};

pub mod insert;
pub mod models;
pub mod schema;
pub mod types;
pub mod utils;

use types::DbPool;

pub const MIGRATIONS: EmbeddedMigrations = diesel_migrations::embed_migrations!();

pub fn init_db() -> DbPool {
    let db_url: String = std::env::var("DATABASE_URL").unwrap_or(String::from("data/db.sqlite3"));

    let manager: r2d2::ConnectionManager<SqliteConnection> =
        r2d2::ConnectionManager::<SqliteConnection>::new(db_url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("Database URL should be a valid path for SQLite database file!")
}

pub fn run_migrations(pool: &DbPool) {
    let mut conn = pool
        .get()
        .expect("Failed to get DB connection for migrations");
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run database migrations");
}
