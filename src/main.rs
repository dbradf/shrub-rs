use serde::Deserialize;
use serde_json;
use serde_yaml;
use shrub_rs::models::commands::{function_call, function_call_with_params, ParamValue};
use shrub_rs::models::project::EvgProject;
use shrub_rs::models::task::{EvgTask, TaskDependency};
use shrub_rs::models::variant::{BuildVariant, DisplayTask};
use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum BoolValue {
    Boolean(bool),
    String(String),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum NumValue {
    Number(u64),
    String(String),
}

#[derive(Debug, Deserialize)]
struct Options {
    num_files: NumValue,
    num_tasks: NumValue,
    resmoke_args: String,
    npm_command: String,
    jstestfuzz_vars: String,
    name: String,
    build_variant: String,
    continue_on_failure: BoolValue,
    resmoke_jobs_max: NumValue,
    should_shuffle: BoolValue,
    timeout_secs: NumValue,
    use_multiversion: Option<String>,
    suite: String,
}

fn get_bool_value(bool_value: &BoolValue) -> bool {
    match bool_value {
        BoolValue::Boolean(b) => *b,
        BoolValue::String(s) => (s == "true" || s == "True"),
    }
}

fn get_u64_value(num_value: &NumValue) -> u64 {
    match num_value {
        NumValue::Number(n) => *n,
        NumValue::String(s) => s.parse::<u64>().unwrap(),
    }
}

impl Options {
    pub fn continue_on_failure(&self) -> bool {
        get_bool_value(&self.continue_on_failure)
    }

    pub fn should_shuffle(&self) -> bool {
        get_bool_value(&self.should_shuffle)
    }

    pub fn timeout_secs(&self) -> u64 {
        get_u64_value(&self.timeout_secs)
    }

    pub fn resmoke_jobs_max(&self) -> u64 {
        get_u64_value(&self.resmoke_jobs_max)
    }

    pub fn num_files(&self) -> u64 {
        get_u64_value(&self.num_files)
    }

    pub fn num_tasks(&self) -> u64 {
        get_u64_value(&self.num_tasks)
    }
}

fn name_generated_task(
    parent_name: &str,
    task_index: usize,
    total_tasks: usize,
    variant: &str,
) -> String {
    let index_width = (total_tasks as f32).log10().ceil() as usize;
    format!(
        "{}_{:0fill$}_{}",
        parent_name,
        task_index,
        variant,
        fill = index_width
    )
}

#[test]
fn test_name_generated_task() {
    assert_eq!("hello_0001_", name_generated_task("hello", 1, 1200, ""));
    assert_eq!("hello_1_", name_generated_task("hello", 1, 8, ""));
    assert_eq!(
        "hello_07_variant",
        name_generated_task("hello", 7, 26, "variant")
    );
}

fn build_sub_task(task_name: &str, task_index: usize, options: &Options) -> EvgTask {
    let sub_task_name = name_generated_task(
        task_name,
        task_index,
        options.num_tasks() as usize,
        &options.build_variant,
    );

    let mut run_jstestfuzz_vars = HashMap::with_capacity(2);
    run_jstestfuzz_vars.insert(
        String::from("jstestfuzz_vars"),
        ParamValue::String(format!(
            "--numGeneratedFiles {} {}",
            options.num_files(), options.jstestfuzz_vars
        )),
    );
    run_jstestfuzz_vars.insert(
        String::from("npm_command"),
        ParamValue::String(options.npm_command.to_string()),
    );

    let suite_args = format!("--suites={}", options.suite);
    let mut run_tests_vars = HashMap::with_capacity(7);
    run_tests_vars.insert(
        String::from("continue_on_failure"),
        ParamValue::Bool(options.continue_on_failure()),
    );
    run_tests_vars.insert(
        String::from("resmoke_args"),
        ParamValue::String(format!("{} {}", suite_args, options.resmoke_args)),
    );
    run_tests_vars.insert(
        String::from("resmoke_jobs_max"),
        ParamValue::Number(options.resmoke_jobs_max()),
    );
    run_tests_vars.insert(
        String::from("should_shuffle"),
        ParamValue::Bool(options.should_shuffle()),
    );
    if let Some(task_path) = &options.use_multiversion {
        run_tests_vars.insert(
            String::from("task_path_suffix"),
            ParamValue::String(task_path.to_string()),
        );
    }
    run_tests_vars.insert(
        String::from("timeout_secs"),
        ParamValue::Number(options.timeout_secs()),
    );
    run_tests_vars.insert(
        String::from("task"),
        ParamValue::String(options.name.to_string()),
    );

    let mut commands = vec![function_call("do setup")];
    if let Some(_) = options.use_multiversion {
        commands.push(function_call("configure evergreen api credentials"));
        commands.push(function_call("do multiversion setup"));
    }
    commands.push(function_call("setup jstestfuzz"));
    commands.push(function_call_with_params(
        "run jstestfuzz",
        run_jstestfuzz_vars,
    ));
    commands.push(function_call_with_params(
        "run generated tests",
        run_tests_vars,
    ));

    EvgTask {
        name: sub_task_name,
        commands,
        depends_on: Some(vec![TaskDependency {
            name: "archive_dist_test_debug".to_string(),
            variant: None,
        }]),
        ..Default::default()
    }
}

fn generate_fuzzer_tasks(options: &Options) -> Vec<EvgTask> {
    (0..options.num_tasks())
        .into_iter()
        .map(|i| build_sub_task(&options.name, i as usize, options))
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
        name: options.build_variant.to_string(),
        tasks: task_list.iter().map(|t| t.get_reference(None)).collect(),
        display_tasks: Some(vec![display_task]),
        ..Default::default()
    };

    EvgProject {
        buildvariants: vec![build_variant],
        tasks: task_list,
        ..Default::default()
    }
}

fn main() {
    let fname = env::args().nth(1).expect("Error: Missing expansions file.");
    let contents = read_to_string(fname).unwrap();
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
