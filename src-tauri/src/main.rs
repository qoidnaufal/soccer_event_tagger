// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{GlobalShortcutManager, Manager};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn open() -> String {
    let maybe_file = rfd::AsyncFileDialog::new().pick_file().await;
    if let Some(path) = maybe_file {
        format!("{:?}", path.path())
    } else {
        String::new()
    }
}

#[tauri::command]
fn ctrl_p(app_handle: tauri::AppHandle) -> tauri::Result<()> {
    app_handle
        .global_shortcut_manager()
        .register("CTRL + P", move || log::info!("CTRL + P is pressed"))
        .map_err(|err| tauri::Error::Runtime(err))
}

fn main() -> tauri::Result<()> {
    env_logger::init();
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.app_handle();
            ctrl_p(app_handle)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![open, ctrl_p])
        .run(tauri::generate_context!())
}
