#[tauri::command]
pub fn open() -> (String, String) {
    let maybe_path = rfd::FileDialog::new().pick_file();
    if let Some(path) = maybe_path {
        let path_to_resolve = path.to_str().unwrap();
        (path_to_resolve.to_string(), "stream".to_string())
    } else {
        (String::new(), String::new())
    }
}
