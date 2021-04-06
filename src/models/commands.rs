//! Commands are the basic building blocks of Evergreen tasks.
//!
//! They can either be built-in Evergreen command or functions customized for this project.
//!
//! See Evergreen [documentation](https://github.com/evergreen-ci/evergreen/wiki/Project-Configuration-Files#commands)
//! for more details.
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// AWS S3 Location description.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct S3Location {
    /// S3 bucket.
    pub bucket: String,
    /// Path within S3 bucket.
    pub path: String,
}

/// Description of how to copy an AWS S3 file.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct S3CopyFile {
    /// Location of S3 file to copy.
    pub source: S3Location,
    /// S3 destination to put copy.
    pub destination: S3Location,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_variants: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// Key-Value pair used to create a parameter map.
#[derive(Serialize, Deserialize, Debug)]
pub struct KeyValueParam {
    /// Key of Key-Value pair.
    pub key: String,
    /// Value of Key-Value pair.
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ParamValue {
    Bool(bool),
    String(String),
    Number(u64),
    List(Vec<String>),
    Map(BTreeMap<String, String>),
    KeyValueList(Vec<KeyValueParam>),
    S3CopyList(Vec<S3CopyFile>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ScriptingHarness {
    Python,
    Python2,
    Golang,
    Roswell,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum CommandType {
    Test,
    System,
    Setup,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum S3Visibility {
    Public,
    Private,
    Signed,
    None,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionCall {
    pub func: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vars: Option<BTreeMap<String, ParamValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_secs: Option<u64>,
}

/// Built-in Evergreen Commands.
#[derive(Debug, Serialize, Deserialize)]
pub enum CommandName {
    /// Extract a tar-gzipped file.
    #[serde(alias = "archive.targz_extract")]
    ArchiveTargzExtract,
    /// Create a tar-gzipped file.
    #[serde(alias = "archive.targz_pack")]
    ArchiveTargzPack,

    #[serde(alias = "archive.auto_extract")]
    ArchiveAutoExtract,
    #[serde(alias = "attach.artifacts")]
    AttachArtifacts,
    #[serde(alias = "attach.results")]
    AttachResults,
    #[serde(alias = "attach.xunit_results")]
    AttachXUnitResults,
    #[serde(alias = "expansions.update")]
    ExpansionsUpdate,
    #[serde(alias = "expansions.write")]
    ExpansionsWrite,
    #[serde(alias = "generate.tasks")]
    GenerateTasks,
    #[serde(alias = "git.get_project")]
    GitGetProject,
    #[serde(alias = "gotest.parse_files")]
    GotestParseFiles,
    #[serde(alias = "host.create")]
    HostCreate,
    #[serde(alias = "host.list")]
    HostList,
    #[serde(alias = "json.send")]
    JsonSend,
    #[serde(alias = "keyval.inc")]
    KeyValInc,
    #[serde(alias = "manifest.load")]
    ManifestLoad,
    #[serde(alias = "s3.get")]
    S3Get,
    #[serde(alias = "s3.put")]
    S3Put,
    #[serde(alias = "s3Copy.copy")]
    S3Copy,
    #[serde(alias = "shell.exec")]
    ShellExec,
    #[serde(alias = "subprocess.exec")]
    SubprocessExec,
    #[serde(alias = "subprocess.scripting")]
    SubprocessScripting,
    #[serde(alias = "timeout.update")]
    TimeoutUpdate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuiltInCommand {
    pub command: CommandName,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<BTreeMap<String, ParamValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params_yaml: Option<String>,
    #[serde(alias = "type", skip_serializing_if = "Option::is_none")]
    pub command_type: Option<CommandType>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Command {
    Function(FunctionCall),
    BuiltIn(BuiltInCommand),
}

pub fn function_call(name: &str) -> Command {
    Command::Function(FunctionCall {
        func: name.to_string(),
        vars: None,
        timeout_secs: None,
    })
}

fn add_string(params: &mut BTreeMap<String, ParamValue>, key: &str, value: Option<&str>) {
    if let Some(val) = value {
        params.insert(key.to_string(), ParamValue::String(val.to_string()));
    }
}

fn add_bool(params: &mut BTreeMap<String, ParamValue>, key: &str, value: Option<bool>) {
    if let Some(val) = value {
        params.insert(key.to_string(), ParamValue::Bool(val));
    }
}

fn add_string_list(params: &mut BTreeMap<String, ParamValue>, key: &str, value: &[&str]) {
    params.insert(
        key.to_string(),
        ParamValue::List(value.iter().map(|s| s.to_string()).collect()),
    );
}

fn add_s3_copy_list(params: &mut BTreeMap<String, ParamValue>, key: &str, value: &[S3CopyFile]) {
    params.insert(
        key.to_string(),
        ParamValue::S3CopyList(value.iter().map(|s| s.clone()).collect()),
    );
}

pub fn archive_targz_extract(
    path: &str,
    destination: &str,
    exclude_files: Option<&str>,
) -> BuiltInCommand {
    let mut params = BTreeMap::new();
    add_string(&mut params, "path", Some(path));
    add_string(&mut params, "destination", Some(destination));
    add_string(&mut params, "exclude_files", exclude_files);

    BuiltInCommand {
        command: CommandName::ArchiveTargzExtract,
        params: Some(params),
        params_yaml: None,
        command_type: None,
    }
}

pub fn archive_targz_pack(
    target: &str,
    source_dir: &str,
    include: &[&str],
    exclude_files: Option<&str>,
) -> BuiltInCommand {
    let mut params = BTreeMap::new();
    add_string(&mut params, "target", Some(target));
    add_string(&mut params, "source_dir", Some(source_dir));
    add_string_list(&mut params, "include", include);
    add_string(&mut params, "exclude_files", exclude_files);

    BuiltInCommand {
        command: CommandName::ArchiveTargzPack,
        params: Some(params),
        params_yaml: None,
        command_type: None,
    }
}

pub fn attach_artifacts(files: &[&str], prefix: Option<&str>) -> BuiltInCommand {
    let mut params = BTreeMap::new();
    add_string_list(&mut params, "files", files);
    add_string(&mut params, "prefix", prefix);

    BuiltInCommand {
        command: CommandName::AttachArtifacts,
        params: Some(params),
        params_yaml: None,
        command_type: None,
    }
}

pub fn s3_copy(aws_key: &str, aws_secret: &str, s3_copy_files: &[S3CopyFile]) -> Command {
    let mut params = BTreeMap::new();
    add_string(&mut params, "aws_key", Some(aws_key));
    add_string(&mut params, "aws_secret", Some(aws_secret));
    add_s3_copy_list(&mut params, "s3_copy_files", s3_copy_files);

    Command::BuiltIn(BuiltInCommand {
        command: CommandName::S3Copy,
        params: Some(params),
        params_yaml: None,
        command_type: None,
    })
}

pub fn shell_exec(
    script: &str,
    working_dir: Option<&str>,
    background: Option<bool>,
    silent: Option<bool>,
    continue_on_err: Option<bool>,
    system_log: Option<bool>,
    shell: Option<&str>,
    ignore_stdout: Option<bool>,
    ignore_stderr: Option<bool>,
    redirect_stderr_to_stdout: Option<bool>,
) -> Command {
    let mut params = BTreeMap::new();
    add_string(&mut params, "script", Some(script));
    add_string(&mut params, "working_dir", working_dir);
    add_bool(&mut params, "background", background);
    add_bool(&mut params, "silent", silent);
    add_bool(&mut params, "continue_on_err", continue_on_err);
    add_bool(&mut params, "system_log", system_log);
    add_string(&mut params, "shell", shell);
    add_bool(&mut params, "ignore_stdout", ignore_stdout);
    add_bool(&mut params, "ignore_stderr", ignore_stderr);
    add_bool(
        &mut params,
        "redirect_stderr_to_stdout",
        redirect_stderr_to_stdout,
    );

    Command::BuiltIn(BuiltInCommand {
        command: CommandName::ShellExec,
        params: Some(params),
        params_yaml: None,
        command_type: None,
    })
}
