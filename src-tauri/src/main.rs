#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    fs,
    io::{Read, Write},
    path::PathBuf,
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    println!("hello");
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Draft<'a> {
    draft_cover: &'a str,
    draft_json_file: &'a str,
    draft_name: &'a str,
    draft_root_path: &'a str,
    tm_draft_create: u64,
    tm_draft_modified: u64,
}

#[tauri::command]
fn save_to_settiing(json: &str) -> String {
    let json_str: Vec<Draft> = serde_json::from_str(json).unwrap();
    let mut confiig = fs::File::create("./myapp.json").unwrap();
    confiig.write_all(serde_json::to_string(&json_str).unwrap().as_bytes());
    confiig.sync_data();
    String::from("sucess")
}

#[tauri::command]
fn projects() -> String {
    let path = PathBuf::from("./myapp.json");
    if path.exists() == false {
        return String::from("");
    }
    let mut config_file = fs::File::open(path).unwrap();
    let mut contents = String::new();
    config_file.read_to_string(&mut contents).unwrap();
    contents
}

fn main() {
    println!("main ......");
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, save_to_settiing, projects])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
