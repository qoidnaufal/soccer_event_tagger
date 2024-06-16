// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod stream;

use std::sync::{Arc, Mutex};

fn main() -> tauri::Result<()> {
    env_logger::init();

    let boundary_id = Arc::new(Mutex::new(0));

    tauri::Builder::default()
        .register_uri_scheme_protocol("stream", move |_, request| {
            stream::get_stream_response(request, &boundary_id)
        })
        .setup(|app| {
            db::Database::init(app)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            stream::open,
            db::event_register::insert_data,
            db::event_register::get_all_data,
            db::event_register::delete_by_id,
            db::event_register::delete_all_records,
            db::match_register::register_match_info,
            db::match_register::get_all_match_info,
            db::match_register::get_match_info_by_match_id,
            db::match_register::get_team_info_by_query,
        ])
        .run(tauri::generate_context!())
}
