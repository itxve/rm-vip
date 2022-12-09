#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![warn(unused_must_use)]

use tauri::Manager;

mod config;
mod mstruct;
mod my_command;

#[tauri::command(main)]
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_data_path = tauri::api::path::app_config_dir(&app.config());
            //初始化app config目录
            my_command::init_app_data_path(app_data_path.unwrap());
            Ok(())
        })
        .on_window_event(|golabl_event| match golabl_event.event() {
            tauri::WindowEvent::Destroyed => {}
            _ => {}
        })
        .on_page_load(|window, _| {
            //添加文件监视器
            let app_data_path = tauri::api::path::app_config_dir(&window.config()).unwrap();
            my_command::watch_jy_file_change(app_data_path, move || {
                window.emit_all(config::JY_FILE_CHANGE, 1).unwrap();
            });
        })
        .invoke_handler(tauri::generate_handler![
            my_command::projects,
            my_command::read_file,
            my_command::write_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
