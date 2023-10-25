use anyhow::Result;
use log::debug;
use serde::Deserialize;
use sqlx::{query_as, Pool, Sqlite};

use crate::models::Settings;

#[derive(Debug, Deserialize)]
pub struct UpdateSettingsAttrs {
    pub notifications: Option<bool>,
    pub sound: Option<bool>,
    pub preview: Option<bool>,
}

pub async fn fetch(pool: &Pool<Sqlite>) -> Result<Option<Settings>> {
    let result = query_as::<_, Settings>(r#"SELECT * FROM settings"#)
        .fetch_one(pool)
        .await;

    let row = match result {
        Ok(settings) => Some(settings),
        Err(_) => None,
    };

    Ok(row)
}

pub async fn update(attrs: UpdateSettingsAttrs, pool: &Pool<Sqlite>) -> Result<()> {
    debug!("Update settings: {:?}", attrs);
    sqlx::query(r#"UPDATE settings set notifications = $1, preview = $2, sound = $3"#)
        .bind(attrs.notifications.unwrap_or_else(|| false))
        .bind(attrs.preview.unwrap_or_else(|| false))
        .bind(attrs.sound.unwrap_or_else(|| false))
        .execute(pool)
        .await?;
    Ok(())
}
