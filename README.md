# shrub-rs

A rust package for defining and interpreting [Evergreen](https://github.com/evergreen-ci/evergreen)
project configuration.

## Table of contents

- [shrub-rs](#shrub-rs)
  - [Table of contents](#table-of-contents)
  - [Description](#description)
  - [Getting Help](#getting-help)
    - [What's the right channel to ask my question?](#whats-the-right-channel-to-ask-my-question)
    - [How can I request a change/report a bug in shrub-rs?](#how-can-i-request-a-changereport-a-bug-in-shrub-rs)
    - [What should I include in my ticket or question?](#what-should-i-include-in-my-ticket-or-question)
  - [Installation](#installation)
  - [Usage](#usage)
    - [Interpreting evergreen project configuration](#interpreting-evergreen-project-configuration)
    - [Exporting evergreen project configuration](#exporting-evergreen-project-configuration)
    - [More examples](#more-examples)
  - [Contributor's Guide](#contributors-guide)
    - [High Level Architecture](#high-level-architecture)
    - [Setting up a local development environment](#setting-up-a-local-development-environment)
    - [linting/formatting](#lintingformatting)
    - [Running tests](#running-tests)
    - [Versioning](#versioning)
    - [Code Review](#code-review)
    - [Deployment](#deployment)
  - [Resources](#resources)

## Description

Provide structs to model evergreen project configuration files.

## Getting Help

### What's the right channel to ask my question?

If you have a question about shrub-rs, please mention @dag-on-call in slack channel #evergreen-users,
or email us at dev-prod-dag@mongodb.com.

### How can I request a change/report a bug in shrub-rs?

Create a DAG ticket in Jira.

### What should I include in my ticket or question?

Please include as much information as possible. This can help avoid long information-gathering threads.

Please include the following:

- **Motivation for Request**: Why is this change being requested? (This help us understand the priority and urgency of the request)
- **Context**: Is there any background information we should be aware of for this request?
- **Description**: What would you like investigated or changed?

## Installation

To install, include "shrub-rs" in your projects Cargo.toml dependencies.

## Usage

Shrub provides two-way translation of evergreen project configurations. Meaning you can convert
evergreen project configurations into rust structs or convert rust structs into evergreen project
configuration.

### Interpreting evergreen project configuration

Converting evergreen project configuration into rust structs can make it much easier to analyze
the configuration of an evergreen project.

**Note**: You will likely want to run a project configuration through `evergreen evaluate` before
interpreting it in order to perform some preprocessing that evergreen does.

A simple example:

```rust
use std::path::Path;
use std::process::Command;

use shrub_rs::models::project::EvgProject;

fn get_project_config(location: &Path) -> EvgProject {
    let evg_config_yaml = Command::new("evergreen")
        .args(&["evaluate", location.to_str().unwrap()])
        .output()
        .unwrap();
    EvgProject::from_yaml_str(std::str::from_utf8(&evg_config_yaml.stdout).unwrap()).unwrap()
}

let evg_project = get_project_config(Path::from("path/to/config.yml));
println!("My project has {} build variants", evg_project.build_variants.len());
```

### Exporting evergreen project configuration

Build up evergreen configuration and then exporting it can be very useful when performing dynamic
task generation via evergreen's `generate.tasks`.

A simple example:

```rust
use std::path::Path;
use shrub_rs::models::{project::EvgProject, task::{EvgTask}};

let evg_task = EvgTask {
    name: "My new task".to_string(),
    ..Default::default()
};
let evg_project = EvgProject {
    tasks: vec![evg_task],
    ..Default::default()
};

std::fs::write(Path::from("path/to/target"), serde_json::to_string_pretty(&evg_project).unwrap()).unwrap();
```

### More examples

For a more complex example of how shrub can be used see the [mongo-task-generator](https://github.com/mongodb/mongo-task-generator).

## Contributor's Guide

### High Level Architecture

Shrub-rs provides rust structs to model evergreen project configuration. To understand
the structures provided, review the [evergreen project configuration documentation](https://github.com/evergreen-ci/evergreen/wiki/Project-Configuration-Files)

### Setting up a local development environment

After cloning the repository, simply run `cargo build` to download and build the project.

### linting/formatting

```bash
cargo fmt
cargo clippy
```

### Running tests

```bash
cargo test
```

### Versioning

This project uses [semver](https://semver.org/) for versioning.

Please include a description what is added for each new version in `CHANGELOG.md`.

### Code Review

Code reviews are required on all changes and are done via Github Pull Requests.

### Deployment

Deployment to production is automatically triggered on merges to master.

## Resources

- [Evergreen Project Commands](https://github.com/evergreen-ci/evergreen/wiki/Project-Commands)
- [Evergreen generate.tasks](https://github.com/evergreen-ci/evergreen/wiki/Project-Commands#generatetasks)
- [Python shrub](https://github.com/evergreen-ci/shrub.py)
- [Golang shrub](https://github.com/evergreen-ci/shrub)
- [Typescript shrub](https://github.com/evergreen-ci/shrub.ts)
