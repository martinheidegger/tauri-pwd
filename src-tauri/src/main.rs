// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayEvent};
use keyring::Entry;
use bcrypt::{hash, DEFAULT_COST};

#[tauri::command]
fn save_password(password: String) -> Result<(), String> {
    let hashed_password = hash(password, DEFAULT_COST).map_err(|e| e.to_string())?;
    let entry = Entry::new("com.tauri.dev", "user").map_err(|e| e.to_string())?;
    entry.set_password(&hashed_password).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn get_stored_hash() -> Result<String, String> {
    let entry = Entry::new("com.tauri.dev", "user").map_err(|e| e.to_string())?;
    entry.get_password().map_err(|e| e.to_string())
}

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new().add_item(quit);
    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                let window = app.get_window("main").unwrap();
                if window.is_visible().unwrap() {
                    window.hide().unwrap();
                } else {
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }
            }
            SystemTrayEvent::MenuItemClick { id, .. } => {
                match id.as_str() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    _ => {}
                }
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![save_password, get_stored_hash])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}