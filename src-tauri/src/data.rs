use types::TaggedEvent;

#[tauri::command]
pub fn register(payload: TaggedEvent) -> String {
    log::info!("payload: {:?}", payload);
    String::from("khelloooo")
}
