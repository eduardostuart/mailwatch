use log::{debug, error};
use sqlx::{Pool, Sqlite};

use crate::{
    db,
    imap::{Credentials, Imap},
    models::Account,
};

// Initialize mail watcher
// Spawn a new async task for each account
pub async fn init(pool: &Pool<Sqlite>) -> anyhow::Result<()> {
    debug!("Initializing watcher");
    let accounts = db::account::list_accounts(pool).await?;
    for acc in accounts {
        tokio::spawn(async move { connect(acc.clone()) });
    }
    Ok(())
}

// TODO: channel, get password from keychain or somewhere else
fn connect(acc: Account) {
    let mut imap = Imap::new(
        Credentials {
            server: (acc.server, acc.port),
            username: acc.username.clone(),
            password: acc.password,
        },
        None,
    );

    match imap.connect() {
        Ok(mut session) => {
            imap.check_for_new_messages(&mut session)
                .expect("could not check for new messages");
        }
        Err(e) => {
            let username = &acc.username;
            error!("error while connecting: {} - {:?}", username, e);
        }
    };
}
