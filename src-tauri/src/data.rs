use tauri::State;
use types::TaggedEvent;

#[tauri::command]
pub fn register(payload: TaggedEvent, state: State<types::Database>) {
    let mut db = state.db.lock().unwrap();
    db.insert(payload.uuid, payload);
}

#[tauri::command]
pub fn get_all(state: State<types::Database>) -> Vec<TaggedEvent> {
    let db = state.db.lock().unwrap();
    let mut vec = db.iter().map(|(_, tag)| tag.clone()).collect::<Vec<_>>();
    vec.sort_by_key(|tag| tag.uuid);
    vec
}

#[tauri::command]
pub fn delete(payload: TaggedEvent, state: State<types::Database>) {
    let mut db = state.db.lock().unwrap();
    db.retain(|uuid, _| *uuid != payload.uuid);
}
