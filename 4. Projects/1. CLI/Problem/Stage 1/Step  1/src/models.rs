use std::collections::HashMap;

pub enum Status {
    // add fields (make sure the fields are public)
    // status can be Open, InProgress, Resolved or Closed.
    Open,
    InProgress,
    Resolved,
    Closed
}

pub struct Epic {
    // add fields (make sure the fields are public)
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<i32>
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        // by default the status should be set to open and the stories should be an empty vector
        Self {
            name,
            description,
            status: Status::Open,
            stories: Vec::new()
        }
    }
}

pub struct Story {
    // add fields (make sure the fields are public)
    pub name: String,
    pub description: String,
    pub status: Status
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        // by default the status should be set to open
        Self {
            name,
            description,
            status: Status::Open
        }
    }
}

pub struct DBState {
    // This struct represents the entire db state which includes the last_item_id, epics, and stories
    // add fields (make sure the fields are public)
    pub last_item_id: i32,
    pub epics: HashMap<i32, Epic>,
    pub stories: HashMap<i32, Story>
}