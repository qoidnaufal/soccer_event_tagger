use tauri::State;
use types::TaggedEvent;

#[tauri::command]
pub fn register(payload: TaggedEvent, state: State<types::Database>) -> Vec<TaggedEvent> {
    log::info!("payload: {:?}", payload);
    let mut db = state.db.lock().unwrap();
    db.push(payload);

    db.to_vec()
}

#[tauri::command]
pub fn delete(payload: TaggedEvent, state: State<types::Database>) -> Vec<TaggedEvent> {
    let mut db = state.db.lock().unwrap();
    db.retain(|tag| *tag != payload);

    db.to_vec()
}
