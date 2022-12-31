extern crate rand;

use rand::{thread_rng, Rng};

#[cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn omikuji(name: &str) -> String {
    let mut rng = thread_rng();
    let rand: u32 = rng.gen_range(0..8);
    format!("Hello, {}:{}! You've been greeted from Rust!", name, rand)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![omikuji])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
