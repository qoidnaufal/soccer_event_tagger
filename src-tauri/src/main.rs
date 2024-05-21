// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{GlobalShortcutManager, Manager};
use types::TaggedEvent;

#[tauri::command]
fn register(payload: TaggedEvent) -> String {
    log::info!("payload: {:?}", payload);
    String::from("khelloooo")
}

#[tauri::command]
async fn open(_content: String) -> String {
    let maybe_file = rfd::AsyncFileDialog::new().pick_file().await;
    if let Some(path) = maybe_file {
        format!("{}", path.path().to_str().unwrap())
    } else {
        String::new()
    }
}

#[tauri::command]
fn ctrl_p(app_handle: tauri::AppHandle) -> tauri::Result<()> {
    app_handle
        .global_shortcut_manager()
        .register("CTRL + I", move || log::info!("CTRL + I is pressed"))
        .map_err(|err| tauri::Error::Runtime(err))
}

fn main() -> tauri::Result<()> {
    env_logger::init();
    tauri::Builder::default()
        .setup(|app| {
            let dir = app
                .path_resolver()
                .resolve_resource("../database.db")
                .expect("failed to resolve resource dir");
            log::info!("Dir: {:?}", dir);
            let app_handle = app.app_handle();
            ctrl_p(app_handle)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![open, ctrl_p, register])
        .run(tauri::generate_context!())
}
