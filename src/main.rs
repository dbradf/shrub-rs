use serde::Deserialize;
use serde_json;
use serde_yaml;
use shrub_rs::models::commands::{
    function_call, function_call_with_params, ParamValue
};
use shrub_rs::models::project::EvgProject;
use shrub_rs::models::task::{Task, TaskDependency};
use shrub_rs::models::variant::{BuildVariant, DisplayTask};
use std::collections::HashMap;
use std::fs::read_to_string;


#[derive(Debug, Deserialize)]
struct Options {
    num_files: usize,
    num_tasks: usize,
    resmoke_args: String,
    npm_command: String,
    jstestfuzz_vars: String,
    name: String,
    build_variant: String,
    continue_on_failure: bool,
    resmoke_jobs_max: u64,
    should_shuffle: bool,
    timeout_secs: u64,
    use_multiversion: Option<String>,
    suite: String,
}

fn name_generated_task(parent_name: &str, task_index: usize, total_tasks: usize, variant: &str) -> String {
    let index_width = (total_tasks as f32).log10().ceil() as usize;
    format!("{}_{:0fill$}_{}", parent_name, task_index, variant, fill = index_width)
} 

#[test]
fn test_name_generated_task() {
    assert_eq!("hello_0001_", name_generated_task("hello", 1, 1200, ""));
    assert_eq!("hello_1_", name_generated_task("hello", 1, 8, ""));
    assert_eq!("hello_07_variant", name_generated_task("hello", 7, 26, "variant"));
}

fn build_sub_task(task_name: &str, task_index: usize, options: &Options) -> Task {
    let sub_task_name = name_generated_task(task_name, task_index, options.num_tasks, &options.build_variant);

    let mut run_jstestfuzz_vars = HashMap::with_capacity(2);
    run_jstestfuzz_vars.insert(String::from("jstestfuzz_vars"), ParamValue::String(format!("--numGeneratedFiles {} {}", options.num_files, options.jstestfuzz_vars)));
    run_jstestfuzz_vars.insert(String::from("npm_command"), ParamValue::String(options.npm_command.to_string()));

    let suite_args = format!("--suites={}", options.suite);
    let mut run_tests_vars = HashMap::with_capacity(7);
    run_tests_vars.insert(String::from("continue_on_failure"), ParamValue::Bool(options.continue_on_failure));
    run_tests_vars.insert(String::from("resmoke_args"), ParamValue::String(format!("{} {}", suite_args, options.resmoke_args)));
    run_tests_vars.insert(String::from("resmoke_jobs_max"), ParamValue::Number(options.resmoke_jobs_max));
    run_tests_vars.insert(String::from("should_shuffle"), ParamValue::Bool(options.should_shuffle));
    if let Some(task_path) = &options.use_multiversion {
        run_tests_vars.insert(String::from("task_path_suffix"), ParamValue::String(task_path.to_string()));
    }
    run_tests_vars.insert(String::from("timeout_secs"), ParamValue::Number(options.timeout_secs));
    run_tests_vars.insert(String::from("task"), ParamValue::String(options.name.to_string()));

    let mut commands = vec![
            function_call("do setup"),
    ];
    if let Some(_) = options.use_multiversion {
        commands.push(function_call("configure evergreen api credentials"));
        commands.push(function_call("do multiversion setup"));
    }
    commands.push(function_call("setup jstestfuzz"));
    commands.push(function_call_with_params("run jstestfuzz", run_jstestfuzz_vars));
    commands.push(function_call_with_params("run generated tests", run_tests_vars));

    Task {
        name: sub_task_name,
        commands: commands,
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

fn generate_fuzzer_tasks(options: &Options) -> Vec<Task> {
    (0..options.num_tasks)
        .into_iter()
        .map(|i| build_sub_task(&options.name, i, options))
        .collect()
}

fn create_project(options: &Options) -> EvgProject {
    let task_list = generate_fuzzer_tasks(options);
    let mut execution_tasks: Vec<String> = task_list.iter().map(|t| t.name.to_string()).collect();
    execution_tasks.push(format!("{}_gen", options.name));
    let display_task = DisplayTask {
        name: options.name.clone(),
        execution_tasks,
    };
        

    let build_variant = BuildVariant {
        name: "build 1".to_string(),
        tasks: task_list.iter().map(|t| t.get_reference(None)).collect(),
        display_tasks: Some(vec![display_task]),
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
    let contents = read_to_string("/home/dbradf/Documents/expansions.yml").unwrap();
    let options: Options = serde_yaml::from_str(&contents).unwrap();

    let p = create_project(&options);
    println!("{}", serde_json::to_string_pretty(&p).unwrap());

    // let c = archive_targz_pack("test.targz", "/hi", &vec!["one", "two", "three"], None);
    //
    // println!("{}", serde_json::to_string(&c).unwrap());

    // let contents = read_to_string("/home/dbradf/Documents/evergreen_parsed.yml").unwrap();
    
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
