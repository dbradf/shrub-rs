//! Evergreen Project are the top level configuration for an Evergreen landscape.
//!
//! See Evergreen's
//! [documentation](https://github.com/evergreen-ci/evergreen/wiki/Project-Configuration-Files)
//! for more details on how a projects configuration.
use crate::models::builtin::EvgCommandType;
use crate::models::commands::EvgCommand;
use crate::models::task::EvgTask;
use crate::models::task_group::EvgTaskGroup;
use crate::models::variant::BuildVariant;
use serde::{Deserialize, Serialize};
use simple_error::bail;
use std::{collections::HashMap, error::Error};
use yaml_merge_keys::merge_keys;
use yaml_rust::{YamlEmitter, YamlLoader};

/// Description of an evergreen parameter.
///
/// Parameters allow patch builds to specific customized behavior.
/// See Evergreen's
/// [Parameterized Builds](https://github.com/evergreen-ci/evergreen/wiki/Parameterized-Builds)
/// documentation for more information.
#[derive(Serialize, Deserialize, Debug, Clone)]
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
#[derive(Serialize, Deserialize, Debug, Clone)]
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

/// Definition of an Evergreen function.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum FunctionDefinition {
    /// Function composed of a single Evergreen command.
    SingleCommand(EvgCommand),
    /// Function composed of several Evergreen commands.
    CommandList(Vec<EvgCommand>),
}

/// Description of an Evergreen Project.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct EvgProject {
    /// List of build variants belonging to this landscape.
    pub buildvariants: Vec<BuildVariant>,
    /// List of task definitions.
    pub tasks: Vec<EvgTask>,
    /// List of task group definitions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_groups: Option<Vec<EvgTaskGroup>>,
    /// Definitions of functions belonging to this landscape.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub functions: HashMap<String, FunctionDefinition>,
    /// List of commands to run at the start of each task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre: Option<Vec<EvgCommand>>,
    /// List of commands to run at the end of each task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post: Option<Vec<EvgCommand>>,
    /// List of commands to run whenever a task hits a timeout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<Vec<EvgCommand>>,

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
    pub command_type: Option<EvgCommandType>,
    /// List of globs that describe file changes that won't trigger a new build.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore: Option<Vec<String>>,
    /// Parameters that can be specified to customize patch build functionality.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<EvgParameter>>,
}

impl EvgProject {
    /// Parse the given YAML string into an Evergreen Project.
    pub fn from_yaml_str(yaml_contents: &str) -> Result<EvgProject, Box<dyn Error>> {
        // Evergreen config can use merge-keys, which is not supported by
        // serde-yaml, so we need to merge the keys first.
        let mut raw = YamlLoader::load_from_str(yaml_contents)?;
        if raw.len() != 1 {
            bail!("Expected 1 and only 1 yaml document")
        }
        let raw = raw.remove(0);
        let merged = merge_keys(raw)?;

        let mut out_str = String::new();
        {
            let mut emitter = YamlEmitter::new(&mut out_str);
            emitter.dump(&merged)?;
        }

        Ok(serde_yaml::from_str(&out_str)?)
    }

    /// Build a map of the defined build variants.
    pub fn build_variant_map(&self) -> HashMap<String, &BuildVariant> {
        let mut map = HashMap::with_capacity(self.buildvariants.len());
        self.buildvariants.iter().for_each(|bv| {
            map.insert(bv.name.to_string(), bv);
        });
        map
    }

    /// Build a map of the defined tasks.
    pub fn task_def_map(&self) -> HashMap<String, &EvgTask> {
        let mut map = HashMap::with_capacity(self.tasks.len());
        self.tasks.iter().for_each(|t| {
            map.insert(t.name.to_string(), t);
        });
        map
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_an_empty_document_fails() {
        let document = "";

        let result = EvgProject::from_yaml_str(document);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_yaml_fails() {
        let document = "garbage input";

        let result = EvgProject::from_yaml_str(document);
        assert!(result.is_err());
    }
}
