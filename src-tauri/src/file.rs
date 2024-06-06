#[tauri::command]
pub async fn open(_content: String) -> (String, String) {
    let maybe_file = rfd::AsyncFileDialog::new().pick_file().await;
    if let Some(file) = maybe_file {
        let path_to_resolve = file.path().to_str().unwrap();
        (path_to_resolve.to_string(), "stream".to_string())
    } else {
        (String::new(), String::new())
    }
}
