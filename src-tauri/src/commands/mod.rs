use flume::Sender;

use crate::{ChannelCmd, UnboundedChannel};

pub mod account;
pub mod connection;
pub mod settings;

/// Send a message to restart the watcher
fn send_restart_watcher_cmd(sender: &Sender<UnboundedChannel>) {
    sender
        .clone()
        .send((ChannelCmd::RestartWatcher, None))
        .expect("Error while sending message");
}
