// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod data;
mod file;
mod stream;

use std::sync::{Arc, Mutex};
use tauri::{GlobalShortcutManager, Manager};

#[tauri::command]
fn ctrl_p(app_handle: tauri::AppHandle) -> tauri::Result<()> {
    app_handle
        .global_shortcut_manager()
        .register("CTRL + I", move || log::info!("CTRL + I is pressed"))
        .map_err(|err| tauri::Error::Runtime(err))
}

fn main() -> tauri::Result<()> {
    env_logger::init();

    let boundary_id = Arc::new(Mutex::new(0));

    tauri::Builder::default()
        .register_uri_scheme_protocol("stream", move |_app_handle, request| {
            stream::get_stream_response(request, &boundary_id)
        })
        .setup(|app| {
            let app_handle = app.app_handle();
            ctrl_p(app_handle)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![file::open, ctrl_p, data::register])
        .run(tauri::generate_context!())
}
