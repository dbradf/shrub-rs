use crate::models::commands::Command;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskDependency {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskRef {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distros: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub name: String,
    pub commands: Vec<Command>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<Vec<TaskDependency>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec_timeout_secs: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patchable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stepback: Option<bool>,
}

impl Task {
    pub fn get_reference(&self, distros: Option<Vec<String>>) -> TaskRef {
        TaskRef {
            name: self.name.clone(),
            distros,
        }
    }
}
