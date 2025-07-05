use std::process::Command;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn copy_to_clipboard(text: String, app_handle: tauri::AppHandle) -> Result<(), String> {
    Command::new("wl-copy")
        .arg(text)
        .output()
        .expect("failed to execute process");

    app_handle.exit(0);

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![copy_to_clipboard])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
