// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{sync::mpsc::channel, thread};

use log::{error, info};
use sqlx::SqlitePool;
use tauri::{
    App, AppHandle, CustomMenuItem, GlobalWindowEvent, Manager, PhysicalPosition, PhysicalSize,
    SystemTray, SystemTrayEvent, SystemTrayMenu, Window,
};

pub mod db;
pub mod error;
pub mod handlers;
pub mod imap;
pub mod keychain;
pub mod models;
pub mod watcher;

pub struct AppState {
    pub pool: SqlitePool,
    pub app_dir: String,
    pub db_dir: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    simple_logger::init_with_level(log::Level::Info)?;

    info!("Initializing app");

    // Directories
    let app_dir = format!("{}/mailwatch", &dirs::home_dir().unwrap().display());
    let db_dir = format!("{}/.db", app_dir);

    // Initialize db
    let db_path = db::initialize_db(std::path::Path::new(&db_dir));
    let pool = db::connect(&db_path).await;
    db::run_migrations(&pool).await?;

    let (tx, rx) = channel::<(models::Account, String)>();
    // Initialize watcher
    watcher::init(&pool, tx)
        .await
        .expect("Error while initializing watcher");

    let rxt = thread::spawn(move || {
        while let Ok((acc, msg)) = rx.recv() {
            info!("Received:{}", msg);
            // TODO: this needs to be improved
            // show total messages?
            // preview of message(s)? (check settings)
            match notify_rust::Notification::new()
                .summary(&acc.name)
                .body("New email received")
                .show()
            {
                Ok(_) => info!("Notification sent"),
                Err(e) => error!("Notification error: {}", e),
            };
        }
    });

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            handlers::account::cmd_create_account,
            handlers::account::cmd_list_accounts,
            handlers::account::cmd_find_account,
            handlers::connection::cmd_test_connection
        ])
        .manage(AppState {
            pool,
            app_dir,
            db_dir,
        })
        .system_tray(build_tray_icon())
        .on_system_tray_event(on_system_tray_event)
        .on_window_event(on_window_event)
        .setup(on_app_setup)
        .build(tauri::generate_context!())
        .expect("Error while running application")
        .run(|_, _| {});

    rxt.join().unwrap();

    Ok(())
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
    match event.event() {
        tauri::WindowEvent::CloseRequested { api, .. } => {
            event.window().hide().unwrap();
            api.prevent_close();
        }
        tauri::WindowEvent::Focused(false) => {
            event.window().hide().unwrap();
        }
        _ => {}
    };
}

fn on_app_setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    // Hide menu from Dock
    #[cfg(target_os = "macos")]
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);

    Ok(())
}
