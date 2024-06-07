// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod data;
mod file;
mod stream;

use std::sync::{Arc, Mutex};
use tauri::{GlobalShortcutManager, Manager};

fn op() {
    let maybe_path = rfd::FileDialog::new().pick_file();

    if let Some(path) = maybe_path {
        log::info!("{:?}", path);
    }
}

fn ctrl_o(app_handle: tauri::AppHandle) -> tauri::Result<()> {
    app_handle
        .global_shortcut_manager()
        .register("CTRL + O", move || op())
        .map_err(|err| tauri::Error::Runtime(err))
}

fn ctrl_i(app_handle: tauri::AppHandle) -> tauri::Result<()> {
    app_handle
        .global_shortcut_manager()
        .register("CTRL + I", move || log::info!("CTRL + I"))
        .map_err(tauri::Error::Runtime)
}

fn main() -> tauri::Result<()> {
    env_logger::init();

    let boundary_id = Arc::new(Mutex::new(0));

    tauri::Builder::default()
        .register_uri_scheme_protocol("stream", move |_, request| {
            stream::get_stream_response(request, &boundary_id)
        })
        .setup(|app| {
            let app_handle = app.app_handle();
            ctrl_o(app_handle.clone())?;
            ctrl_i(app_handle)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![file::open, data::register])
        .run(tauri::generate_context!())
}
