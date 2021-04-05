use crate::models::commands::{Command, CommandType};
use crate::models::task::Task;
use crate::models::variant::BuildVariant;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct EvgParameter {
    pub key: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EvgModule {
    name: String,
    branch: String,
    repo: String,
    prefix: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EvgProject {
    pub buildvariants: Vec<BuildVariant>,
    pub tasks: Vec<Task>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub functions: BTreeMap<String, Vec<Command>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre: Option<Vec<Command>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post: Option<Vec<Command>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<Vec<Command>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub modules: Option<Vec<EvgModule>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stepback: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_error_fails_task: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oom_tracker: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_type: Option<CommandType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<EvgParameter>>,
}
