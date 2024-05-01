use log::info;
use serde::Deserialize;
use tauri::{command, State, Window};

use crate::{async_cmd, db::settings, error::Error, models::Settings, AppState};

#[derive(Debug, Deserialize)]
pub struct UpdateSettingsAttrs {
    pub notifications: Option<bool>,
    pub sound: Option<bool>,
    pub preview: Option<bool>,
}

#[command]
pub async fn cmd_update_settings(
    attrs: UpdateSettingsAttrs,
    _window: Window,
    state: State<'_, AppState>,
) -> Result<(), Error> {
    info!("Updating settings: {:?}", attrs);
    async_cmd!(settings::update(
        settings::UpdateSettingsAttrs {
            notifications: attrs.notifications,
            sound: attrs.sound,
            preview: attrs.preview
        },
        &state.pool,
    ))
    .await
}

#[command]
pub async fn cmd_fetch_settings(
    _window: Window,
    state: State<'_, AppState>,
) -> Result<Option<Settings>, Error> {
    async_cmd!(settings::fetch(&state.pool)).await
}
