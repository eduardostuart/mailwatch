use std::{cmp, net::TcpStream, sync::mpsc::Sender};

use anyhow::{anyhow, Result};
use imap::Session;
use log::{error, info};
use native_tls::{TlsConnector, TlsStream};

use crate::{
    keychain::{Keychain, KeychainEntryKey},
    models::{self, Account},
};

#[derive(Debug)]
pub struct Credentials {
    /// Server host and port
    pub server: (String, i64),
    /// Server username
    pub username: String,
    /// Server password
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct Imap {
    /// Account information
    account: Option<Account>,
}

impl Imap {
    pub fn new(account: Option<Account>) -> Self {
        Self { account }
    }

    /// connect will initialize a new `Client<TlsStream<TcpStream>>` client
    /// login and verify server capabilities (IDLE support)
    pub fn connect_account(&mut self) -> Result<Session<TlsStream<TcpStream>>> {
        if self.account.is_none() {
            return Err(anyhow!("Invalid account"));
        }

        let acc = self.account.clone().unwrap();

        let password = Keychain::new()
            .get_password(KeychainEntryKey::new(acc.id, &acc.username))
            .expect("Password not found");

        self.connect(
            &acc.server,
            acc.port,
            &acc.username,
            &password,
            &acc.mailbox,
        )
    }

    pub fn connect(
        &mut self,
        server: &str,
        port: i64,
        username: &str,
        password: &str,
        mailbox: &str,
    ) -> Result<Session<TlsStream<TcpStream>>> {
        let ssl_connector = TlsConnector::builder().build()?;
        let addr = format!("{}:{}", server, port);
        let client = imap::connect(addr, server, &ssl_connector)?;
        match client.login(username, password) {
            Ok(mut session) => {
                // Check if the server has IDLE capability
                // IDLE capability can be used to receive notifications of new messages without polling.
                // https://datatracker.ietf.org/doc/html/rfc2177
                let support_idle = session.capabilities()?.has_str("IDLE");
                if !support_idle {
                    let _ = session.logout();
                    return Err(anyhow!("server does not support IDLE "));
                }
                let _ = session.select(mailbox);
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
        tx: &Sender<(models::Account, String)>,
    ) -> Result<()> {
        if self.account.is_none() {
            return Err(anyhow!("Invalid account"));
        }

        let mut last_notified = 0;
        let acc = self.account.clone().unwrap();

        loop {
            if tx.send((acc.clone(), "Message".to_string())).is_err() {
                break Ok(());
            }

            info!("Checking account: {}", acc.username);
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
    pub fn test_connection(
        &mut self,
        server: &str,
        port: i64,
        username: &str,
        password: &str,
        mailbox: &str,
    ) -> Result<String> {
        info!("Testing connection");
        match self.connect(server, port, username, password, mailbox) {
            Ok(mut session) => {
                session.logout()?;
                Ok("OK".to_string())
            }
            Err(e) => {
                error!("Error from test connection: {:?}", e);
                Err(anyhow!("Error: {:?}", e))
            }
        }
    }
}
