#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![warn(unused_must_use)]

mod command;
mod config;
mod mstruct;

fn main() {
    let app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            command::projects,
            command::read_file,
            command::write_file
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");
    //初始化app config目录
    command::init_app_data_path(tauri::api::path::app_config_dir(&app.config()).unwrap());
    app.run(|_app_handle, _event| {});
}
