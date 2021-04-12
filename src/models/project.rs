//! Evergreen Project are the top level configuration for an Evergreen landscape.
//!
//! See Evergreen's
//! [documentation](https://github.com/evergreen-ci/evergreen/wiki/Project-Configuration-Files)
//! for more details on how a projects configuration.
use crate::models::builtin::CommandType;
use crate::models::commands::Command;
use crate::models::task::EvgTask;
use crate::models::variant::BuildVariant;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Description of an evergreen parameter.
///
/// Parameters allow patch builds to specific customized behavior.
/// See Evergreen's
/// [Parameterized Builds](https://github.com/evergreen-ci/evergreen/wiki/Parameterized-Builds)
/// documentation for more information.
#[derive(Serialize, Deserialize, Debug)]
pub struct EvgParameter {
    /// Name of parameter.
    pub key: String,
    /// Default value to use for parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Description of parameter.
    pub description: String,
}

/// Description of a module to include in a landscape.
#[derive(Serialize, Deserialize, Debug)]
pub struct EvgModule {
    /// Name of module being defined.
    pub name: String,
    /// Repository containing module to be included.
    pub repo: String,
    /// Branch of repository to use.
    pub branch: String,
    /// Path to store module code at.
    pub prefix: String,
}

/// Description of an Evergreen Project.
#[derive(Serialize, Deserialize, Debug)]
pub struct EvgProject {
    /// List of build variants belonging to this landscape.
    pub buildvariants: Vec<BuildVariant>,
    /// List of task definitions.
    pub tasks: Vec<EvgTask>,
    /// Definitions of functions belonging to this landscape.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub functions: HashMap<String, Vec<Command>>,
    /// List of commands to run at the start of each task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre: Option<Vec<Command>>,
    /// List of commands to run at the end of each task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post: Option<Vec<Command>>,
    /// List of commands to run whenever a task hits a timeout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<Vec<Command>>,

    /// Description of modules to include in this landscape.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modules: Option<Vec<EvgModule>>,

    /// Describe if skipped tasks should be run on failures to determine source of failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stepback: Option<bool>,
    /// Describe if failures in `pre` commands should cause a task to be failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_error_fails_task: Option<bool>,
    /// Describe if evergreen should track out of memory failure in this landscape.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oom_tracker: Option<bool>,
    /// Describe the type of failure a task failure should trigger.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_type: Option<CommandType>,
    /// List of globs that describe file changes that won't trigger a new build.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore: Option<Vec<String>>,
    /// Parameters that can be specified to customize patch build functionality.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<EvgParameter>>,
}

impl Default for EvgProject {
    fn default() -> Self {
        EvgProject {
            buildvariants: vec![],
            tasks: vec![],
            functions: Default::default(),
            pre: None,
            post: None,
            timeout: None,
            modules: None,
            stepback: None,
            pre_error_fails_task: None,
            oom_tracker: None,
            command_type: None,
            ignore: None,
            parameters: None,
        }
    }
}
