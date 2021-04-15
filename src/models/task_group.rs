use serde::{Deserialize, Serialize};

use super::{builtin::TimeoutValue, commands::EvgCommand};


/// A group of tasks related tasks that can share hosts.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EvgTaskGroup {
    /// Name of task group.
    pub name: String,
    /// Ordered list of tasks to include in group.
    pub tasks: Vec<String>,

    /// Number of hosts to spread group accross.
    /// Defaults to 1, can be between 1 and 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_hosts: Option<u16>,
    /// Don't cleanup between task runs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_processes: Option<bool>,
    /// Setup group failures will trigger failures.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_group_can_fail_task: Option<bool>,
    /// Time to wait until setup will trigger a failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_group_timeout_secs: Option<TimeoutValue>,

    /// Commands to run prior to running task group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_group: Option<Vec<EvgCommand>>,
    /// Commands to run after running task group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teardown_group: Option<Vec<EvgCommand>>,
    /// Commands to run before each task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_task: Option<Vec<EvgCommand>>,
    /// Commands to run after each task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teardown_task: Option<Vec<EvgCommand>>,
    /// Commands to run in case of timeout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<Vec<EvgCommand>>,
}
