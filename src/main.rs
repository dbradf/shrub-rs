use serde_json;
use serde_yaml;
use shrub_rs::models::commands::{
    archive_targz_extract, archive_targz_pack, function_call, Command, FunctionCall, ParamValue,
};
use shrub_rs::models::project::EvgProject;
use shrub_rs::models::task::{Task, TaskDependency};
use shrub_rs::models::variant::BuildVariant;
use std::fs::read_to_string;

fn build_sub_task(task_name: &str, task_index: usize) -> Task {
    Task {
        name: format!("{}_{}", task_name, task_index),
        commands: vec![
            function_call("do setup"),
            function_call("configure evergreen api credentials"),
            function_call("do multiversion setup"),
            function_call("setup jstestfuzz"),
            function_call("run jstestfuzz"),
            function_call("run generated tests"),
        ],
        depends_on: Some(vec![TaskDependency {
            name: "archive_dist_test_debug".to_string(),
            variant: None,
        }]),
        exec_timeout_secs: None,
        tags: None,
        patchable: None,
        stepback: None,
    }
}

fn generate_fuzzer_tasks(task_name: &str, count: usize) -> Vec<Task> {
    (0..count)
        .into_iter()
        .map(|i| build_sub_task(task_name, i))
        .collect()
}

fn create_project(task_name: &str, count: usize) -> EvgProject {
    let task_list = generate_fuzzer_tasks(task_name, count);
    let build_variant = BuildVariant {
        name: "build 1".to_string(),
        tasks: task_list.iter().map(|t| t.get_reference(None)).collect(),
        display_name: None,
        run_on: None,
        batchtime: None,
        expansions: None,
        stepback: None,
        modules: None,
    };

    EvgProject {
        buildvariants: vec![build_variant],
        tasks: task_list,
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

fn main() {
    let p = create_project("agg_fuzzer", 10);
    println!("{}", serde_yaml::to_string(&p).unwrap());

    // let c = archive_targz_pack("test.targz", "/hi", &vec!["one", "two", "three"], None);
    //
    // println!("{}", serde_json::to_string(&c).unwrap());

    // let contents = read_to_string("/home/dbradf/Downloads/evergreen_parsed.yml").unwrap();
    //
    // let p: EvgProject = serde_yaml::from_str(&contents).unwrap();
    // println!("Build Variants: {}", p.buildvariants.len());
    // for (name, def) in p.functions {
    //     println!("{}", name);
    //     for c in def {
    //         if let Command::BuiltIn(b) = c {
    //             if b.command == "shell.exec" {
    //                 let params = b.params.unwrap();
    //                 let script = params.get("script").unwrap();
    //                 if let ParamValue::String(s) = script {
    //                     println!("{}", s);
    //                 }
    //             }
    //         }
    //     }
    // }
}
