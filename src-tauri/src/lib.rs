// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn copy_to_clipboard(text: String, app_handle: tauri::AppHandle) -> Result<(), String> {
    use tauri_plugin_clipboard_manager::ClipboardExt;

    app_handle
        .clipboard()
        .write_text(text)
        .map_err(|e| e.to_string())?;

    // Close the application after copying
    app_handle.exit(0);

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .invoke_handler(tauri::generate_handler![copy_to_clipboard])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
