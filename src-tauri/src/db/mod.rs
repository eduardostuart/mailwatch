pub mod account;

use sqlx::{Pool, Sqlite, SqlitePool};
use std::path::Path;

/// Database name
const DB_NAME: &str = "mailwatch.db";

/// Establishes connection to the SQLite database using the specified database path.
///
/// # Arguments
/// * `db_path` - The file path to the SQLite database.
///
/// # Returns
/// Returns a connection pool (`Pool<Sqlite>`) for interacting with the SQLite database.
///
/// # Panics
/// Panics if there is an error while connecting to the database.
pub async fn connect(db_path: &str) -> Pool<Sqlite> {
    SqlitePool::connect(format!("sqlite://{}", db_path).as_str())
        .await
        .expect("Error while connecting to db")
}

/// Runs database migrations
///
/// # Arguments
/// * `pool` - the connection pool to the SQLite database.
/// ```
pub async fn run_migrations(pool: &Pool<Sqlite>) -> anyhow::Result<()> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}

/// Constructs the path to the database file
///
/// # Arguments
/// * `db_dir` - A reference to the path of the database directory.
///
/// # Returns
/// Returns the path to the database file as a String.
///
/// # Example
/// ```rust
/// let db_dir = Path::new("/path/to/db");
/// let db_file_path = db_path(&db_dir);
/// ```
pub fn db_path(db_dir: &Path) -> String {
    db_dir.join(DB_NAME).display().to_string()
}

/// Initializes the database
///
/// # Arguments
/// * `db_dir` - A reference to the path of the database directory.
///
/// # Returns
/// Returns the path to the database file as a String.
///
/// # Panics
/// Panics if the directory or the database file cannot be created.
///
/// # Example
/// ```rust
/// let db_dir = Path::new("/path/to/db");
/// let db_path = initialize_db(&db_dir);
/// ```
pub fn initialize_db(db_dir: &Path) -> String {
    if !db_dir.exists() {
        std::fs::create_dir_all(db_dir).unwrap();
    }

    // Tries to create the db file if it does not exist
    let db_path = db_path(db_dir);
    if !Path::new(&db_path).exists() {
        std::fs::File::create(&db_path).unwrap();
    }

    db_path
}
