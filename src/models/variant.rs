use crate::models::task::TaskRef;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Representation of an Evergreen Build Variant.
#[derive(Serialize, Deserialize, Debug)]
pub struct BuildVariant {
    /// Name of build variant.
    pub name: String,
    /// List of tasks to add to build variant.
    pub tasks: Vec<TaskRef>,
    /// Display name of build variant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// List of distros tasks run on by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_on: Option<Vec<String>>,
    /// How frequently tasks should be run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batchtime: Option<u64>,
    /// Map of expansions that should be passed to tasks at runtime.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expansions: Option<BTreeMap<String, String>>,
    /// Should failed tasks be run on skipped versions to find their source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stepback: Option<bool>,
    /// List of modules that should be included in tasks for this build variant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modules: Option<Vec<String>>,
}
