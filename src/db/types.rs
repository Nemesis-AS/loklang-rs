use diesel::{r2d2, SqliteConnection};

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;
pub type DbError = Box<dyn std::error::Error + Send + Sync>;