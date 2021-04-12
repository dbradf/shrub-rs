//! Commands are the basic building blocks of Evergreen tasks.
//!
//! They can either be built-in Evergreen command or functions customized for this landscape.
//!
//! See Evergreen [documentation](https://github.com/evergreen-ci/evergreen/wiki/Project-Configuration-Files#commands)
//! for more details.
use crate::models::builtin::BuiltInCommand;
use crate::models::params::ParamValue;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionCall {
    pub func: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vars: Option<HashMap<String, ParamValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_secs: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Command {
    Function(FunctionCall),
    BuiltIn(BuiltInCommand),
}

pub fn function_call(name: &str) -> Command {
    Command::Function(FunctionCall {
        func: name.to_string(),
        vars: None,
        timeout_secs: None,
    })
}

pub fn function_call_with_params(name: &str, vars: HashMap<String, ParamValue>) -> Command {
    Command::Function(FunctionCall {
        func: String::from(name),
        vars: Some(vars),
        timeout_secs: None,
    })
}
