use crate::config;
use crate::mstruct::{DraftItem, DraftStore, InitType, Rt};
use std::{fs, io::Read, path::PathBuf};

#[tauri::command]
pub fn projects(app_config_path: std::path::PathBuf) -> Rt<Vec<DraftItem>> {
    let path = PathBuf::from(app_config_path).join(config::JY_CONFIG_FILE);
    if path.exists() == false {
        return Rt {
            data: vec![] as Vec<DraftItem>,
            err: String::from("Ok"),
        };
    }
    let mut config_file = fs::File::open(path).unwrap();
    let mut contents = String::new();
    config_file.read_to_string(&mut contents).unwrap();
    let draft: DraftStore = serde_json::from_str(&contents).unwrap();
    Rt {
        data: draft.all_draft_store,
        err: String::from("Ok"),
    }
}

#[tauri::command]
pub fn read_file(file_path: std::path::PathBuf) -> Rt<Vec<u8>> {
    std::fs::read(file_path).map_or_else(
        |err| Rt {
            data: vec![],
            err: err.to_string(),
        },
        |data| Rt {
            data,
            err: String::from(""),
        },
    )
}

#[tauri::command]
pub fn write_file(file_path: std::path::PathBuf, data: &str) -> Rt<String> {
    std::fs::write(file_path, data).map_or_else(
        |err| Rt {
            data: "".to_string(),
            err: err.to_string(),
        },
        |_| Rt {
            data: "".to_string(),
            err: "".to_string(),
        },
    )
}

#[tauri::command]
pub fn init_app_data_path(file_path: std::path::PathBuf) -> InitType {
    if file_path.exists() {
        InitType::EXIST
    } else {
        fs::DirBuilder::new()
            .recursive(true)
            .create(file_path)
            .map_or_else(|_| InitType::CreateError, |_| InitType::SUCCESS)
    }
}
