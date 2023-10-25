use anyhow::Result;
use serde::Deserialize;
use sqlx::{query_as, Pool, Sqlite};

use crate::models::Account;

#[derive(Debug, Deserialize)]
pub struct CreateAccountAttrs {
    pub name: String,
    pub server: String,
    pub port: i64,
    pub color: String,
    pub active: bool,
    pub username: String,
    pub password: String,
}

/// Creates a new account in the database and returns its unique identifier.
///
/// # Arguments
/// * `attrs` - `CreateAccountAttrs` account creation attributes
/// * `pool` - A reference to the SQLite connection pool.
pub async fn create_account(attrs: CreateAccountAttrs, pool: &Pool<Sqlite>) -> Result<i64> {
    let query = r#"
        INSERT INTO accounts 
            (name, server, port, color, active, username, password) 
        VALUES 
            ($1,$2,$3,$4,$5,$6,$7)
    "#;

    let id = sqlx::query(query)
        .bind(attrs.name)
        .bind(attrs.server)
        .bind(attrs.port)
        .bind(attrs.color)
        .bind(attrs.active)
        .bind(attrs.username)
        .bind(attrs.password)
        .execute(pool)
        .await?
        .last_insert_rowid();

    Ok(id)
}

/// List all accounts ordered by id in descending order
///
/// # Arguments
/// * `pool` - A reference to the SQLite connection pool.
pub async fn list_accounts(pool: &Pool<Sqlite>) -> Result<Vec<Account>> {
    // TODO: remove password
    let query = r#"SELECT 
        id, name, color, server, port, active, username, password
        FROM accounts 
        order by id 
        desc"#;

    Ok(sqlx::query_as::<_, Account>(query).fetch_all(pool).await?)
}

/// Find an account by its id
///
/// # Arguments
/// * `id` - The account id to be retrieved
/// * `pool` - A reference to the SQLite connection pool.
pub async fn find_account_by_id(id: i64, pool: &Pool<Sqlite>) -> Result<Account> {
    let query = r#"SELECT 
        id, name, color, server, port, active, username 
    FROM 
     accounts where id = ?
    "#;

    let result = query_as::<_, Account>(query)
        .bind(id)
        .fetch_one(pool)
        .await?;

    Ok(result)
}
