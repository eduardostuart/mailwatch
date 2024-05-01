use log::{error, info};
use serde::Deserialize;
use tauri::{command, State};

use crate::{async_cmd, db::account, error::Error, keychain::Keychain, models::Account, AppState};

use super::send_restart_watcher_cmd;

#[derive(Debug, Deserialize)]
pub struct NewAccountAttrs<'a> {
    pub name: &'a str,
    pub server: &'a str,
    pub color: &'a str,
    pub active: bool,
    pub username: &'a str,
    pub password: &'a str,
    pub mailbox: &'a str,
    pub port: i64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAccountAttrs<'a> {
    pub name: &'a str,
    pub server: &'a str,
    pub color: &'a str,
    pub username: &'a str,
    pub mailbox: &'a str,
    pub port: i64,
    pub password: Option<&'a str>,
}

/// Command to create new acounts
#[command]
pub async fn cmd_create_account(
    attrs: NewAccountAttrs<'_>,
    state: State<'_, AppState>,
) -> Result<i64, Error> {
    let account = async_cmd!(account::create(
        account::CreateAccountAttrs {
            name: attrs.name,
            server: attrs.server,
            port: attrs.port,
            color: attrs.color,
            active: true,
            username: attrs.username,
            mailbox: attrs.mailbox,
        },
        &state.pool
    ))
    .await?;

    match Keychain::new(account.id, &account.username).new_entry(attrs.password) {
        Ok(_) => info!("New value added to keychain"),
        Err(e) => {
            error!("Something went wrong, deleting account entry {}", e);
            account::delete(account.id, &state.pool)
                .await
                .expect("error while deleting");
        }
    }

    send_restart_watcher_cmd(&state.sender);

    Ok(account.id)
}

/// Command to list all accounts ordered by id desc
#[command]
pub async fn cmd_list_accounts(state: State<'_, AppState>) -> Result<Vec<Account>, Error> {
    async_cmd!(account::all(&state.pool)).await
}

/// Command to find accounts by id
#[command]
pub async fn cmd_find_account(id: i64, state: State<'_, AppState>) -> Result<Account, Error> {
    async_cmd!(account::find(id, &state.pool)).await
}

#[command]
pub async fn cmd_delete_account(id: i64, state: State<'_, AppState>) -> Result<(), Error> {
    let acc = async_cmd!(account::find(id, &state.pool)).await?;

    let key = Keychain::new(acc.id, &acc.username).get_entry();
    if key.is_ok() && key.unwrap().delete_password().is_ok() {
        info!("Key deleted from keychain");
    }

    async_cmd!(account::delete(id, &state.pool)).await?;

    send_restart_watcher_cmd(&state.sender);

    Ok(())
}

#[command]
pub async fn cmd_update_account(
    id: i64,
    attrs: UpdateAccountAttrs<'_>,
    state: State<'_, AppState>,
) -> Result<(), Error> {
    let acc = async_cmd!(account::find(id, &state.pool)).await?;

    async_cmd!(account::update(
        id,
        account::UpdateAccountAttrs {
            name: attrs.name,
            server: attrs.server,
            port: attrs.port,
            color: attrs.color,
            username: attrs.username,
            mailbox: attrs.mailbox,
        },
        &state.pool,
    ))
    .await?;

    // Update keychain value if the user has provided a new password
    if let Some(pwd) = attrs.password {
        if !pwd.trim().is_empty() {
            match Keychain::new(acc.id, &acc.username).new_entry(pwd) {
                Ok(_) => info!("password updated"),
                Err(_) => error!("error while updating password"),
            }
        }
    }

    send_restart_watcher_cmd(&state.sender);

    Ok(())
}
