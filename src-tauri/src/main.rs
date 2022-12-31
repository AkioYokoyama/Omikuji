extern crate rand;

use rand::{thread_rng, Rng};

#[cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn omikuji() -> String {
    let mut rng = thread_rng();
    let rand: u32 = rng.gen_range(0..7);

    let lot: String = match rand % 7 {
        0 => return String::from("大吉"),
        1 => return String::from("吉"),
        2 => return String::from("中吉"),
        3 => return String::from("小吉"),
        4 => return String::from("末吉"),
        5 => return String::from("凶"),
        6 => return String::from("大凶"),
        _ => String::from(""),
    };

    return lot;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![omikuji])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
