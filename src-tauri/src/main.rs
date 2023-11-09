// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use db::{initialize_db, run_migrations};
use flume::{unbounded, Sender};
use lazy_static::lazy_static;
use log::{debug, error, info, LevelFilter};
use simple_logger::SimpleLogger;
use sqlx::{Pool, Sqlite, SqlitePool};
use tauri::{
    api::{notification::Notification, path::home_dir},
    App, AppHandle, CustomMenuItem, GlobalWindowEvent, Manager, PhysicalPosition, PhysicalSize,
    SystemTray, SystemTrayEvent, SystemTrayMenu, Window,
};

use crate::{db::account, watcher::Watcher};

pub mod commands;
pub mod db;
pub mod error;
pub mod imap;
pub mod keychain;
pub mod macros;
pub mod models;
pub mod watcher;

lazy_static! {
    static ref APP_DIR: String = format!("{}/mailwatch", home_dir().unwrap().display());
    static ref DB_DIR: String = format!("{}/.db", APP_DIR.as_str());
}

pub struct AppState {
    pub pool: SqlitePool,
    pub sender: Sender<UnboundedChannel>,
}

#[derive(Debug, Clone)]
pub enum ChannelCmd {
    Notify,
    RestartWatcher,
}

pub type UnboundedChannel = (ChannelCmd, Option<models::Account>);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .expect("Failed to initialize logger");

    // Initialize database
    let db_path = initialize_db(std::path::Path::new(&DB_DIR.as_str()));
    let pool = db::connect(&db_path).await;
    run_migrations(&pool)
        .await
        .expect("Error while running migrations");

    let (tx, rx) = unbounded::<UnboundedChannel>();

    let mut watcher = Watcher::new(tx.clone());
    start_watcher(&mut watcher, &pool).await;

    let pool_clone: Pool<Sqlite> = pool.clone();
    let rtx = tokio::spawn(async move {
        debug!("Starting to check for new messages.");
        while let Ok((cmd, acc)) = rx.recv() {
            info!("Command received: {:?}", cmd);
            match cmd {
                ChannelCmd::Notify => {
                    if let Some(account) = acc {
                        let notification = Notification::new(&account.name)
                            .body("New email received")
                            .title(&account.name);

                        if let Err(e) = notification.show() {
                            error!("Failed to show notification for {}: {}", account.name, e);
                        }
                    }
                }
                ChannelCmd::RestartWatcher => {
                    restart_watcher(&mut watcher, &pool_clone).await;
                }
            };
        }
    });

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::cmd_create_account,
            commands::cmd_list_accounts,
            commands::cmd_find_account,
            commands::cmd_delete_account,
            commands::cmd_update_account,
            commands::cmd_test_connection
        ])
        .manage(AppState {
            pool,
            sender: tx.clone(),
        })
        .system_tray(build_tray_icon())
        .on_system_tray_event(on_system_tray_event)
        .on_window_event(on_window_event)
        .setup(on_app_setup)
        .build(tauri::generate_context!())?
        .run(|_, _| {});

    if let Err(e) = rtx.await {
        error!("Watcher task terminated: {:?}", e);
    }

    Ok(())
}

/// Shutdown and start the watcher again
///
/// The purpose of this function is to restart the watcher when the user
/// adds, deletes or update an existing account
async fn restart_watcher(watcher: &mut Watcher, pool: &Pool<Sqlite>) {
    let _ = &watcher.shutdown();
    start_watcher(watcher, pool).await;
}

async fn start_watcher(watcher: &mut Watcher, pool: &Pool<Sqlite>) {
    info!("Starting watcher");
    let accounts = account::all(pool).await.unwrap();
    let _ = &watcher.start(accounts);
}

fn build_tray_icon() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit").accelerator("Cmd+Q");
    let tray_menu = SystemTrayMenu::new().add_item(quit);
    SystemTray::new().with_menu(tray_menu)
}

fn on_system_tray_event(app: &AppHandle, e: SystemTrayEvent) {
    match e {
        tauri::SystemTrayEvent::MenuItemClick { id, .. } => {
            if id.eq("quit") {
                std::process::exit(0);
            }
        }
        // On left click toggle the window visibility
        // Show menu on left click should be disabled on tauri config
        tauri::SystemTrayEvent::LeftClick {
            tray_id: _,
            position,
            size,
            ..
        } => {
            toggle_window_visibility(&app.get_window("main").unwrap(), position, size);
        }
        _ => {}
    };
}

fn toggle_window_visibility(
    window: &Window,
    position: PhysicalPosition<f64>,
    size: PhysicalSize<f64>,
) {
    match window.is_visible().unwrap() {
        true => window.hide().unwrap(),
        false => {
            let window_size = window.outer_size().unwrap();
            let physical_pos = PhysicalPosition {
                x: position.x as i32 + (size.width as i32 / 2) - (window_size.width as i32 / 2),
                y: position.y as i32 - window_size.height as i32,
            };
            let _ = window.set_position(tauri::Position::Physical(physical_pos));
            window.show().unwrap();
            window.set_focus().unwrap();
        }
    };
}

// Window event to prevent killing the app when the user clicks on "close"
// This will hide the window and keep it on tray
// The user can still kill the app using CMD+Q or tray "quit" option
fn on_window_event(event: GlobalWindowEvent) {
    if let tauri::WindowEvent::CloseRequested { api, .. } = event.event() {
        event.window().hide().unwrap();
        api.prevent_close();
    }
}

fn on_app_setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    // Hide menu from Dock
    #[cfg(target_os = "macos")]
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);

    Ok(())
}
