use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_yaml;
use shrub_rs::models::commands::{archive_targz_extract, archive_targz_pack, Command, ParamValue};
use std::fs::read_to_string;
use shrub_rs::models::project::EvgProject;

fn main() {
    // let c = archive_targz_pack("test.targz", "/hi", &vec!["one", "two", "three"], None);
    //
    // println!("{}", serde_json::to_string(&c).unwrap());

    let contents = read_to_string("/home/dbradf/Downloads/evergreen_parsed.yml").unwrap();

    let p: EvgProject = serde_yaml::from_str(&contents).unwrap();
    println!("Build Variants: {}", p.buildvariants.len());
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
