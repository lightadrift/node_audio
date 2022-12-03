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

mod devices_intefaces;
mod events_handler;
use devices_intefaces::get_devices::Device;
use tauri::Manager;
use windows::Win32::Media::Audio::{eAll, DEVICE_STATE_ACTIVE};
use events_handler::ui::get_data;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let device = Device {};
            let test1 = device.get_list(eAll, DEVICE_STATE_ACTIVE);
            println!("{:?}", test1);
            let main_window = app.get_window("main").unwrap();
            main_window.set_decorations(false).expect("msg");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, test, get_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
