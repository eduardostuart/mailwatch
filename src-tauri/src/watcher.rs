use std::sync::mpsc::Sender;

use log::{debug, error, info};
use sqlx::{Pool, Sqlite};

use crate::{
    db,
    imap::Imap,
    models::{self, Account},
};

// Initialize mail watcher
// Spawn a new async task for each account
pub async fn init(
    pool: &Pool<Sqlite>,
    tx: Sender<(models::Account, String)>,
) -> anyhow::Result<()> {
    let accounts = db::account::list_accounts(pool).await?;
    for acc in accounts {
        let tx_clone = tx.clone();
        tokio::spawn(async move { connect(acc.clone(), tx_clone) });
    }
    Ok(())
}

fn connect(acc: Account, tx: Sender<(models::Account, String)>) {
    let mut imap = Imap::new(Some(acc.clone()));
    match imap.connect_account() {
        Ok(mut session) => {
            info!("{} - connected", &acc.username);
            imap.check_for_new_messages(&mut session, &tx)
                .expect("could not check for new messages");
        }
        Err(e) => {
            let username = &acc.username;
            error!("error while connecting: {} - {:?}", username, e);
        }
    };
}
