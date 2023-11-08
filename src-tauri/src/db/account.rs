use anyhow::Result;
use serde::Deserialize;
use sqlx::{query_as, Pool, Sqlite};

use crate::models::Account;

#[derive(Debug, Deserialize)]
pub struct CreateAccountAttrs<'a> {
    pub name: &'a str,
    pub server: &'a str,
    pub port: i64,
    pub color: &'a str,
    pub active: bool,
    pub username: &'a str,
    pub mailbox: &'a str,
}

/// Creates a new account in the database and returns its unique identifier.
///
/// # Arguments
/// * `attrs` - `CreateAccountAttrs` account creation attributes
/// * `pool` - A reference to the SQLite connection pool.
pub async fn create_account(attrs: CreateAccountAttrs<'_>, pool: &Pool<Sqlite>) -> Result<Account> {
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
    sqlx::query(r#"DELETE from accounts where id = $1"#)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

/// List all accounts ordered by id in descending order
///
/// # Arguments
/// * `pool` - A reference to the SQLite connection pool.
pub async fn list_accounts(pool: &Pool<Sqlite>) -> Result<Vec<Account>> {
    let result = sqlx::query_as::<_, Account>(
        r#"
        SELECT id, name, color, server, port, active, username, mailbox
        FROM accounts 
        ORDER BY id desc
    "#,
    )
    .fetch_all(pool)
    .await?;
    Ok(result)
}

/// Find an account by id
///
/// # Arguments
/// * `id` - The account id to be retrieved
/// * `pool` - A reference to the SQLite connection pool.
pub async fn find_account_by_id(id: i64, pool: &Pool<Sqlite>) -> Result<Account> {
    let result = query_as::<_, Account>(
        r#"
        SELECT id, name, color, server, port, active, username, mailbox
        FROM accounts 
        WHERE id = ?
    "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(result)
}

#[derive(Debug, Deserialize)]
pub struct UpdateAccountAttrs<'a> {
    pub name: &'a str,
    pub server: &'a str,
    pub port: i64,
    pub color: &'a str,
    pub username: &'a str,
    pub mailbox: &'a str,
}

pub async fn update_account(
    id: i64,
    attrs: UpdateAccountAttrs<'_>,
    pool: &Pool<Sqlite>,
) -> Result<()> {
    sqlx::query(
        r#"
        UPDATE accounts  
        SET name = $1, color = $2, server = $3, port = $4, username = $5, mailbox = $6
        WHERE id = $7
    "#,
    )
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
