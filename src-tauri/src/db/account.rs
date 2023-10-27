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
    pub mailbox: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAccountAttrs {
    pub name: String,
    pub server: String,
    pub port: i64,
    pub color: String,
    pub username: String,
    pub mailbox: String,
}

/// Creates a new account in the database and returns its unique identifier.
///
/// # Arguments
/// * `attrs` - `CreateAccountAttrs` account creation attributes
/// * `pool` - A reference to the SQLite connection pool.
pub async fn create_account(attrs: CreateAccountAttrs, pool: &Pool<Sqlite>) -> Result<Account> {
    let query = r#"
        INSERT INTO accounts 
            (name, server, port, color, active, username, mailbox) 
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
        .bind(attrs.mailbox)
        .execute(pool)
        .await?
        .last_insert_rowid();

    let account = find_account_by_id(id, pool).await?;

    Ok(account)
}

/// Delete an account
///
/// # Arguments
/// * `id` - account id to be deleted
pub async fn delete_account(id: i64, pool: &Pool<Sqlite>) -> Result<()> {
    let query = r#"DELETE from accounts where id = $1"#;
    sqlx::query(query).bind(id).execute(pool).await?;
    Ok(())
}

/// List all accounts ordered by id in descending order
///
/// # Arguments
/// * `pool` - A reference to the SQLite connection pool.
pub async fn list_accounts(pool: &Pool<Sqlite>) -> Result<Vec<Account>> {
    // TODO: remove password
    let query = r#"SELECT 
            id, name, color, server, port, active, username, mailbox
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
        id, name, color, server, port, active, username, mailbox
    FROM 
     accounts where id = ?
    "#;

    let result = query_as::<_, Account>(query)
        .bind(id)
        .fetch_one(pool)
        .await?;

    Ok(result)
}

pub async fn update_account(id: i64, attrs: UpdateAccountAttrs, pool: &Pool<Sqlite>) -> Result<()> {
    let query = r#"
        UPDATE accounts  
            set name = $1, color = $2, server = $3, port = $4, username = $5, mailbox = $6
        where id = $7
    "#;

    sqlx::query(query)
        .bind(attrs.name)
        .bind(attrs.color)
        .bind(attrs.server)
        .bind(attrs.port)
        .bind(attrs.username)
        .bind(attrs.mailbox)
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}
