use tauri::{command, State};

use crate::{
    db::account::{create_account, find_account_by_id, list_accounts, CreateAccountAttrs},
    error::Error,
    models::Account,
    AppState,
};

use super::execute_async_command;

/// Command to create new acounts
#[command]
pub async fn cmd_create_account(
    attrs: CreateAccountAttrs,
    state: State<'_, AppState>,
) -> Result<i64, Error> {
    println!("cmd_create_account");
    execute_async_command(create_account(attrs, &state.pool)).await
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
