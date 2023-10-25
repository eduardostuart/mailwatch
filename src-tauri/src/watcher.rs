use flume::Sender;
use log::{error, info, warn};
use tokio::task::JoinHandle;

use crate::{imap::Imap, models::Account, UnboundedChannel};

#[derive(Debug)]
pub struct Watcher {
    tx: Sender<UnboundedChannel>,
    threads: Vec<JoinHandle<()>>,
}

impl Watcher {
    pub fn new(tx: Sender<UnboundedChannel>) -> Self {
        Self {
            tx,
            threads: vec![],
        }
    }

    pub fn start(&mut self, accounts: Vec<Account>) {
        info!("Starting watcher with {} accounts.", accounts.len());

        let thread_pool = tokio::runtime::Handle::current();

        self.threads = accounts
            .into_iter()
            .map(|acc| {
                let tx = self.tx.clone();
                thread_pool.spawn(async move {
                    Watcher::watch_account(&acc, tx);
                })
            })
            .collect();
    }

    fn watch_account(acc: &Account, tx: Sender<UnboundedChannel>) {
        info!("{} - starting", acc.username);
        let mut imap = Imap::new(Some(acc));

        match imap
            .connect_account()
            .and_then(|mut s| imap.check_for_new_messages(&mut s, &tx))
        {
            Ok(_) => info!("Finished checking for messages: {}", acc.username),
            Err(e) => error!(
                "Error while checking messages for account {}: {:?}",
                acc.username, e
            ),
        }
    }

    pub fn shutdown(&mut self) {
        warn!("shutting down watcher");
        for thread in self.threads.drain(..) {
            thread.abort();
        }
    }
}
