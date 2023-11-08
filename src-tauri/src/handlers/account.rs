use flume::Sender;
use log::{error, info};
use serde::Deserialize;
use tauri::{command, State};

use crate::{
    async_cmd,
    db::{
        self,
        account::{
            create_account, delete_account, find_account_by_id, list_accounts, update_account,
            CreateAccountAttrs,
        },
    },
    error::Error,
    keychain::{Keychain, KeychainEntryKey},
    models::Account,
    AppState, Command, UnboundedChannel,
};

#[derive(Debug, Deserialize)]
pub struct NewAccountAttrs<'a> {
    pub name: &'a str,
    pub server: &'a str,
    pub port: i64,
    pub color: &'a str,
    pub active: bool,
    pub username: &'a str,
    pub password: &'a str,
    pub mailbox: &'a str,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAccountAttrs<'a> {
    pub name: &'a str,
    pub server: &'a str,
    pub port: i64,
    pub color: &'a str,
    pub username: &'a str,
    pub mailbox: &'a str,
    pub password: Option<&'a str>,
}

/// Command to create new acounts
#[command]
pub async fn cmd_create_account(
    attrs: NewAccountAttrs<'_>,
    state: State<'_, AppState>,
) -> Result<i64, Error> {
    let account = async_cmd!(create_account(
        CreateAccountAttrs {
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

    let keychain_entry = &KeychainEntryKey::new(account.id, &account.username);
    match Keychain::new().new_entry(keychain_entry, attrs.password) {
        Ok(_) => info!("New value added to keychain"),
        Err(e) => {
            error!("Something went wrong, deleting account entry {}", e);
            delete_account(account.id, &state.pool)
                .await
                .expect("error while deleting");
        }
    }

    restart_watcher(&state.sender);

    Ok(account.id)
}

/// Command to list all accounts ordered by id desc
#[command]
pub async fn cmd_list_accounts(state: State<'_, AppState>) -> Result<Vec<Account>, Error> {
    async_cmd!(list_accounts(&state.pool)).await
}

/// Command to find accounts by id
#[command]
pub async fn cmd_find_account(id: i64, state: State<'_, AppState>) -> Result<Account, Error> {
    async_cmd!(find_account_by_id(id, &state.pool)).await
}

#[command]
pub async fn cmd_delete_account(id: i64, state: State<'_, AppState>) -> Result<(), Error> {
    let acc = async_cmd!(find_account_by_id(id, &state.pool)).await?;

    let key = Keychain::new().get_entry(&KeychainEntryKey::new(acc.id, &acc.username));

    if key.is_ok() && key.unwrap().delete_password().is_ok() {
        info!("Key deleted from keychain");
    }

    async_cmd!(delete_account(id, &state.pool)).await?;

    restart_watcher(&state.sender);

    Ok(())
}

#[command]
pub async fn cmd_update_account(
    id: i64,
    attrs: UpdateAccountAttrs<'_>,
    state: State<'_, AppState>,
) -> Result<(), Error> {
    let acc = async_cmd!(find_account_by_id(id, &state.pool)).await?;

    async_cmd!(update_account(
        id,
        db::account::UpdateAccountAttrs {
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

    if let Some(pwd) = attrs.password {
        if !pwd.trim().is_empty() {
            let keychain_entry = KeychainEntryKey::new(acc.id, &acc.username);
            let keychain_result = Keychain::new().new_entry(&keychain_entry, pwd);

            match keychain_result {
                Ok(_) => info!("password updated"),
                Err(_) => error!("error while updating password"),
            }
        }
    }

    restart_watcher(&state.sender);

    Ok(())
}

/// Send a message to restart the watcher
fn restart_watcher(sender: &Sender<UnboundedChannel>) {
    sender
        .clone()
        .send((Command::RestartWatcher, None))
        .expect("Error while sending message");
}
