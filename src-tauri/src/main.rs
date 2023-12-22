// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            let _ = main_window.eval("window.location.replace('http://notion.so')");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
