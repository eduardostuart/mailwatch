use std::thread;

use crate::db::{account, settings};
use crate::imap::{ConnectionDetails, Imap};
use crate::models::Settings;
use crate::{
    async_cmd, db::account::CreateAccountAttrs, error::Error, keychain::Keychain, models::Account,
    AppState, ChannelCmd, UnboundedChannel,
};
use flume::Sender;
use log::{error, info};
use serde::Deserialize;
use tauri::{command, State, Window};

const CONNECTION_TEST_EVENT: &str = "connection_test_result";

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

#[derive(Debug, Deserialize, Clone)]
pub struct TestConnectionAttrs<'a> {
    /// Imap server
    pub server: &'a str,
    /// Imap port
    pub port: i64,
    /// Imap username
    pub username: &'a str,
    /// Imap password
    pub password: &'a str,
    /// Mailbox name
    pub mailbox: &'a str,
}

/// Command to create new acounts
#[command]
pub async fn cmd_create_account(
    attrs: NewAccountAttrs<'_>,
    state: State<'_, AppState>,
) -> Result<i64, Error> {
    let account = async_cmd!(account::create(
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

/// A command to verify IMAP connection.
///
/// This test attempts a connection,
/// returning "OK" if the connection is successful, or the server error message on failure.
/// This command runs asynchronously and will emit an event when the connection test ends.
#[command]
pub fn cmd_test_connection(attrs: TestConnectionAttrs<'_>, window: Window) {
    let conn_details = &ConnectionDetails {
        server: (attrs.server, attrs.port),
        username: attrs.username,
        password: attrs.password,
        mailbox: attrs.mailbox,
    };

    thread::scope(|s| {
        s.spawn(|| {
            window
                .emit(
                    CONNECTION_TEST_EVENT,
                    match Imap::test_connection(conn_details) {
                        Ok(msg) => msg,
                        Err(e) => e.to_string(),
                    },
                )
                .unwrap();
        });
    });
}

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

/// Send a message to restart the watcher
fn send_restart_watcher_cmd(sender: &Sender<UnboundedChannel>) {
    sender
        .clone()
        .send((ChannelCmd::RestartWatcher, None))
        .expect("Error while sending message");
}
