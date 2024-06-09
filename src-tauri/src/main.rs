// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
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

fn ctrl_o(app: &mut tauri::App) -> tauri::Result<()> {
    let app_handle = app.app_handle();
    app_handle
        .global_shortcut_manager()
        .register("CTRL + O", move || op())
        .map_err(|err| tauri::Error::Runtime(err))
}

fn main() -> tauri::Result<()> {
    env_logger::init();

    let boundary_id = Arc::new(Mutex::new(0));

    tauri::Builder::default()
        .register_uri_scheme_protocol("stream", move |_, request| {
            stream::get_stream_response(request, &boundary_id)
        })
        .setup(|app| {
            db::Database::init(app)?;
            ctrl_o(app)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            file::open,
            db::logic::insert_data,
            db::logic::get_all_data,
            db::logic::delete_by_id,
        ])
        .run(tauri::generate_context!())
}
