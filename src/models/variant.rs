use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;


#[derive(Serialize, Deserialize, Debug)]
pub struct TaskRef {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    distros: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BuildVariant {
    pub name: String,
    pub tasks: Vec<TaskRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_on: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batchtime: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expansions: Option<BTreeMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stepback: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modules: Option<Vec<String>>,


}

// def as_dict(self) -> Dict[str, Any]:
// """Get the dictionary representation of this build variant."""
// obj: Dict[str, Any] = {
// "name": self.name,
// "tasks": self.__get_task_specs(frozenset(self.tasks))
// + self.__get_task_specs(frozenset(self.task_groups))
// + self.__get_task_specs(frozenset(self.existing_tasks)),
// }
//
// if self.display_tasks:
// obj["display_tasks"] = sorted(
// [dt.as_dict() for dt in self.display_tasks], key=lambda d: d["name"]
// )
//
// add_existing_from_dict(
// obj,
// {
// "expansions": self.expansions,
// "run_on": self.run_on,
// "modules": self.modules,
// "display_name": self.display_name,
// "batch_time": self.batch_time,
// },
// )