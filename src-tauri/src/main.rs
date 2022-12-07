#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![warn(unused_must_use)]

mod command;
mod config;
mod mstruct;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            command::projects,
            command::read_file,
            command::init_app_data_path,
            command::write_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
