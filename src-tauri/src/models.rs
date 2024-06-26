use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, FromRow, Debug, Deserialize, Clone)]
pub struct Account {
    pub id: i64,
    pub name: String,
    pub server: String,
    pub port: i64,
    pub color: String,
    pub active: bool,
    pub username: String,
    pub mailbox: String,
}

#[derive(Serialize, FromRow, Debug, Deserialize, Clone)]
pub struct Settings {
    pub notifications: Option<bool>,
    pub sound: Option<bool>,
    pub preview: Option<bool>,
}
