use std::{cmp, net::TcpStream};

use anyhow::{anyhow, Result};
use imap::Session;
use log::debug;
use native_tls::{TlsConnector, TlsStream};

#[derive(Debug)]
pub struct Credentials {
    /// Server host and port
    pub server: (String, i64),
    /// Server username
    pub username: String,
    /// Server password
    pub password: String,
}

#[derive(Debug)]
pub struct Imap<'a> {
    /// Credential details
    creds: Credentials,
    /// Inbox "folder/label/..." name
    mailbox: &'a str,
}

impl<'a> Imap<'a> {
    pub fn new(creds: Credentials, mailbox: Option<&'a str>) -> Self {
        let mailbox_name = if let Some(mailbox) = mailbox {
            mailbox
        } else {
            "INBOX"
        };

        Self {
            creds,
            mailbox: mailbox_name,
        }
    }

    /// connect will initialize a new `Client<TlsStream<TcpStream>>` client
    /// login and verify server capabilities (IDLE support)
    pub fn connect(&mut self) -> Result<Session<TlsStream<TcpStream>>> {
        let ssl_connector = TlsConnector::builder().build()?;
        let addr = format!("{}:{}", &self.creds.server.0, &self.creds.server.1);
        let client = imap::connect(addr, &self.creds.server.0, &ssl_connector)?;
        match client.login(&self.creds.username, &self.creds.password) {
            Ok(mut session) => {
                // Check if the server has IDLE capability
                // IDLE capability can be used to receive notifications of new messages without polling.
                // https://datatracker.ietf.org/doc/html/rfc2177
                let support_idle = session.capabilities()?.has_str("IDLE");
                if !support_idle {
                    let _ = session.logout();
                    return Err(anyhow!("server does not support IDLE "));
                }
                let _ = session.select(self.mailbox);
                Ok(session)
            }
            Err((e, _)) => Err(anyhow!(e.to_string())),
        }
    }

    /// Initialize idle checker
    /// For every new message we notify our main thread
    /// which will be responsible to update the systray icon and show a desktop notification
    ///
    /// Here we also keep track of the last notified message
    /// TODO: include channel  to notify about new messages
    pub fn check_for_new_messages(
        &self,
        session: &mut Session<TlsStream<TcpStream>>,
    ) -> Result<()> {
        let mut last_notified = 0;

        loop {
            debug!("Checking account: {}", self.creds.username);
            let mut new_uids = session.uid_search("NEW 1:*").expect("new ids");

            if new_uids.iter().all(|&uid| uid <= last_notified) {
                new_uids.clear();
            }

            last_notified = cmp::max(last_notified, new_uids.iter().cloned().max().unwrap_or(0));
            session.idle()?.wait_keepalive()?;
        }
    }

    /// Simple connection test
    /// Validate if imap connection are correct (server, credentials and mailbox)
    pub fn test_connection(&mut self) -> Result<String> {
        match self.connect() {
            Ok(mut session) => {
                session.logout()?;
                Ok("OK".to_string())
            }
            Err(e) => Err(anyhow!("Error: {:?}", e)),
        }
    }
}
