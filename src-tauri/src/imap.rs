use std::{cmp, collections::HashMap, net::TcpStream, sync::Mutex};

use crate::{keychain::Keychain, models::Account, ChannelCmd, UnboundedChannel};
use anyhow::{anyhow, Result};
use flume::Sender;
use imap::Session;
use lazy_static::lazy_static;
use log::{error, info};
use native_tls::{TlsConnector, TlsStream};

lazy_static! {
    static ref LAST_NOTIFIED: Mutex<HashMap<i64, u32>> = Mutex::new(HashMap::new());
}

#[derive(Debug, Clone)]
pub struct ConnectionDetails<'a> {
    /// Server host and port
    pub server: (&'a str, i64),
    /// Server username
    pub username: &'a str,
    /// Server password
    pub password: &'a str,
    /// Mailbox name
    pub mailbox: &'a str,
}

#[derive(Debug, Clone)]
pub struct Imap<'ac> {
    /// Account information
    account: Option<&'ac Account>,
}

impl<'ac> Imap<'ac> {
    pub fn new(account: Option<&'ac Account>) -> Self {
        Self { account }
    }

    /// connect will initialize a new `Client<TlsStream<TcpStream>>` client
    /// login and verify server capabilities (IDLE support)
    pub fn connect_account(&mut self) -> Result<Session<TlsStream<TcpStream>>> {
        if self.account.is_none() {
            return Err(anyhow!("Invalid account"));
        }

        let acc = self.account.unwrap();

        let password = Keychain::new(acc.id, &acc.username)
            .get_password()
            .expect("Password not found");

        self.connect(&ConnectionDetails {
            server: (&acc.server, acc.port),
            username: &acc.username,
            password: &password,
            mailbox: &acc.mailbox,
        })
    }

    pub fn connect(&mut self, conn: &ConnectionDetails) -> Result<Session<TlsStream<TcpStream>>> {
        let ssl_connector = TlsConnector::builder().build()?;
        let client = imap::connect(
            format!("{}:{}", conn.server.0, conn.server.1),
            conn.server.0,
            &ssl_connector,
        )?;
        match client.login(conn.username, conn.password) {
            Ok(mut session) => {
                // Check if the server has IDLE capability
                // IDLE capability can be used to receive notifications of new messages without polling.
                // https://datatracker.ietf.org/doc/html/rfc2177
                let support_idle = session.capabilities()?.has_str("IDLE");
                if !support_idle {
                    let _ = session.logout();
                    return Err(anyhow!("server does not support IDLE "));
                }
                let _ = session.select(conn.mailbox);
                Ok(session)
            }
            Err((e, _)) => Err(anyhow!(e.to_string())),
        }
    }

    /// Initialize idle checker
    /// For every new message we send a message to the channel
    /// which will be responsible to update the systray icon and show a desktop notification
    /// Here we also keep track of the last notified message
    pub fn check_for_new_messages(
        &self,
        session: &mut Session<TlsStream<TcpStream>>,
        tx: &Sender<UnboundedChannel>,
    ) -> Result<()> {
        if self.account.is_none() {
            return Err(anyhow!("Invalid account"));
        }

        let acc = self.account.unwrap();

        info!("Starting watcher for account: {}", acc.username);

        loop {
            let mut last_notified_history = LAST_NOTIFIED.lock().unwrap();

            info!("Messages checked for account: {:?}", &last_notified_history);
            if tx.send((ChannelCmd::Notify, Some(acc.clone()))).is_err() {
                error!("Err while sending message. stopping watcher");
                break Ok(());
            }

            info!("Checking account: {}", acc.username);

            let mut new_uids = session.uid_search("NEW 1:*").expect("new ids");
            let mut last_notified = match last_notified_history.get(&acc.id) {
                Some(v) => *v,
                None => 0,
            };

            if new_uids.iter().all(|&uid| uid <= last_notified) {
                new_uids.clear();
            }

            last_notified = cmp::max(last_notified, new_uids.iter().cloned().max().unwrap_or(0));
            last_notified_history.insert(acc.id, last_notified);
            session.idle()?.wait_keepalive()?;
        }
    }

    /// Simple connection test
    /// Validate if imap connection are correct (server, credentials and mailbox)
    pub fn test_connection(conn_details: &ConnectionDetails) -> Result<String> {
        info!("Testing connection");
        match Imap::new(None).connect(conn_details) {
            Ok(mut session) => {
                info!("test_connection - Connection successful");
                session.logout()?;
                Ok("OK".to_string())
            }
            Err(e) => {
                error!("test_connection - Error: {:?}", e);
                Err(anyhow!("Error: {:?}", e))
            }
        }
    }
}
