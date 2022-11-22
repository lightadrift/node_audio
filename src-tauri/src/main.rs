#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Helloa, {}!", name)
}

#[tauri::command]
fn test(_name: &str) {
    println!("foi")
}

use tauri::Manager;

fn main() {
    tauri::Builder::default().setup(|app| {
    let main_window = app.get_window("main").unwrap();
    let _ = main_window.set_decorations(false);
      Ok(())

  }).invoke_handler(tauri::generate_handler![greet, test])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
