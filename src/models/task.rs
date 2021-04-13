//! An Evergreen task is a single unit of work that Evergreen performs. Tasks are frequently
//! mapped to test suites, but can task be used for other purposes (like builds or linting).
//!
//! Tasks are build from a list of commands that are either
//! [built-in evergreen commands](https://github.com/evergreen-ci/evergreen/wiki/Project-Commands)
//! or
//! [functions](https://github.com/evergreen-ci/evergreen/wiki/Project-Configuration-Files#functions)
//! unique to the landscape.
use crate::models::commands::EvgCommand;
use serde::{Deserialize, Serialize};

/// Description of a depedency for a task.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskDependency {
    /// Name of task that needs to be run.
    pub name: String,
    /// Build variant where dependent task is run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
}

/// Reference to a task that is being added to a build variant.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskRef {
    /// Name of task.
    pub name: String,
    /// List of distros that task should be run on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distros: Option<Vec<String>>,
}

/// Definition of an Evergreen task.
#[derive(Serialize, Deserialize, Debug)]
pub struct EvgTask {
    /// Name of task being defined.
    pub name: String,
    /// List of command that make up the task.
    pub commands: Vec<EvgCommand>,
    /// List of other tasks that need to be completed before this is done.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<Vec<TaskDependency>>,
    /// How long this task can run before timing out (in seconds).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec_timeout_secs: Option<u64>,
    /// List of tags describing this task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Describe if this patch should be runnable in patch builds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patchable: Option<bool>,
    /// Describe if previously skipped versions of this task should be run on failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stepback: Option<bool>,
}

impl EvgTask {
    /// Get a reference of this task to include in a build variant.
    ///
    /// * `distros`: List of distros this task should run on.
    pub fn get_reference(&self, distros: Option<Vec<String>>) -> TaskRef {
        TaskRef {
            name: self.name.clone(),
            distros,
        }
    }
}

impl Default for EvgTask {
    fn default() -> Self {
        EvgTask {
            name: "".to_string(),
            commands: vec![],
            depends_on: None,
            exec_timeout_secs: None,
            tags: None,
            patchable: None,
            stepback: None,
        }
    }
}
