// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn show_path(path: &str) -> String {
   format!(
        "Hello! Your data at {} is being made into a file, generated in Rust!",
        path
    )
}

#[tauri::command]
fn generate_file(data_path: &str) -> () {
    // this call is not not working yet? Why?
    print!("This is just a log message from Rust for now. Selected dataPath is {data_path}");
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::default().build())
        .invoke_handler(tauri::generate_handler![show_path, generate_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
