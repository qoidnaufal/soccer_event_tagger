// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use percent_encoding::{percent_encode, NON_ALPHANUMERIC};
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
    if let Some(file) = maybe_file {
        let path_to_resolve = file.path().to_str().unwrap();
        // let encoded_path = percent_encode(path_to_resolve.as_bytes(), NON_ALPHANUMERIC).to_string();

        let mut resolved_path = String::from("asset://localhost/");
        resolved_path.push_str(path_to_resolve);
        log::info!("resolved path: {}", resolved_path);

        resolved_path
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
        // .register_uri_scheme_protocol("stream", |app_handle, request| {})
        .setup(|app| {
            let app_handle = app.app_handle();
            ctrl_p(app_handle)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![open, ctrl_p, register])
        .run(tauri::generate_context!())
}
