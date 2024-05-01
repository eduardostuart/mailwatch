use std::thread;

use serde::Deserialize;
use tauri::{command, Window};

use crate::imap::{ConnectionDetails, Imap};

const CONNECTION_TEST_EVENT: &str = "connection_test_result";

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
