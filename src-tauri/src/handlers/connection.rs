use crate::imap::{Credentials, Imap};
use serde::Deserialize;
use tauri::{command, Window};

#[derive(Debug, Deserialize)]
pub struct TestConnectionAttrs {
    /// Imap server
    pub server: String,
    /// Imap port
    pub port: i64,
    /// Imap username
    pub username: String,
    /// Imap password
    pub password: String,
}

/// A command to verify IMAP server credentials.
///
/// This test attempts a connection,
/// returning "OK" if the connection is successful, or the server error message on failure.
/// This command runs asynchronously and will emit an event called "connection_test_result"
/// when the connection test ends.
#[command]
pub fn cmd_test_connection(attrs: TestConnectionAttrs, window: Window) {
    let creds = Credentials {
        server: (attrs.server, attrs.port),
        username: attrs.username,
        password: attrs.password,
    };

    tauri::async_runtime::spawn(async move {
        window
            .emit(
                "connection_test_result",
                match Imap::new(creds, None).test_connection() {
                    Ok(msg) => msg,
                    Err(e) => e.to_string(),
                },
            )
            .unwrap();
    });
}
