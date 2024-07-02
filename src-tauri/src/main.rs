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
            db::event_register::get_events_by_match_id,
            db::event_register::delete_by_id,
            db::event_register::delete_all_records_by_match_id,
            db::event_register::delete_all_data,
            db::event_register::export_tagged_events_from_match_id,
            db::event_register::export_all_tagged_events,
            db::clear_db,
            db::match_register::register_match_info,
            db::match_register::register_player_info,
            db::match_register::get_all_match_info,
            db::match_register::get_all_players_from_match_id,
            db::match_register::query_team_info,
            db::match_register::delete_match_info_by_id,
            db::match_register::delete_all_match_info,
        ])
        .run(tauri::generate_context!())
}
