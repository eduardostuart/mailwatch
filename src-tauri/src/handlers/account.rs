use log::{error, info};
use serde::Deserialize;
use tauri::{command, State};

use crate::{
    db::account::{
        create_account, delete_account, find_account_by_id, list_accounts, CreateAccountAttrs,
    },
    error::Error,
    keychain::{Keychain, KeychainEntryKey},
    models::Account,
    AppState,
};

use super::execute_async_command;

#[derive(Debug, Deserialize)]
pub struct NewAccountAttrs {
    pub name: String,
    pub server: String,
    pub port: i64,
    pub color: String,
    pub active: bool,
    pub username: String,
    pub password: String,
    pub mailbox: String,
}

/// Command to create new acounts
#[command]
pub async fn cmd_create_account(
    attrs: NewAccountAttrs,
    state: State<'_, AppState>,
) -> Result<i64, Error> {
    let new_account_attrs = CreateAccountAttrs {
        name: attrs.name,
        server: attrs.server,
        port: attrs.port,
        color: attrs.color,
        active: true,
        username: attrs.username,
        mailbox: attrs.mailbox,
    };

    let account = execute_async_command(create_account(new_account_attrs, &state.pool)).await?;

    let keychain_entry = &KeychainEntryKey::new(account.id, &account.username);
    match Keychain::new().new_entry(keychain_entry, &attrs.password) {
        Ok(_) => info!("New value added to keychain"),
        Err(e) => {
            // TODO: use db transaction
            error!("Something went wrong, deleting account entry {}", e);
            delete_account(account.id, &state.pool)
                .await
                .expect("error while deleting");
        }
    }

    Ok(account.id)
}

/// Command to list all accounts ordered by id desc
#[command]
pub async fn cmd_list_accounts(state: State<'_, AppState>) -> Result<Vec<Account>, Error> {
    execute_async_command(list_accounts(&state.pool)).await
}

/// Command to find accounts by id
#[command]
pub async fn cmd_find_account(id: i64, state: State<'_, AppState>) -> Result<Account, Error> {
    execute_async_command(find_account_by_id(id, &state.pool)).await
}

#[command]
pub async fn cmd_delete_account(id: i64, state: State<'_, AppState>) -> Result<(), Error> {
    let acc = execute_async_command(find_account_by_id(id, &state.pool)).await?;

    let key = Keychain::new().get_entry(&KeychainEntryKey::new(acc.id, &acc.username));

    if key.is_ok() && key.unwrap().delete_password().is_ok() {
        info!("Key deleted from keychain");
    }

    execute_async_command(delete_account(id, &state.pool)).await?;

    Ok(())
}
