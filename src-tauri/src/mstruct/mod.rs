use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DraftItem {
    pub draft_cover: String,
    pub draft_json_file: String,
    pub draft_name: String,
    // pub draft_root_path: String,
    // pub tm_draft_create: u64,
    // pub tm_draft_modified: u64,
    pub tm_duration: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DraftStore {
    pub all_draft_store: Vec<DraftItem>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rt<T> {
    pub data: T,
    pub err: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum InitType {
    EXIST,
    CreateError,
    SUCCESS,
}
