// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    println!("[rust] run greet");
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn my_command(name: &str, time: &str) -> String {
    println!("[rust] run my_command");
    format!("Run my command {} on {}", name, time)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, my_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
