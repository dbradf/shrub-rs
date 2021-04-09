//! Commands are the basic building blocks of Evergreen tasks.
//!
//! They can either be built-in Evergreen command or functions customized for this project.
//!
//! See Evergreen [documentation](https://github.com/evergreen-ci/evergreen/wiki/Project-Configuration-Files#commands)
//! for more details.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    Map(HashMap<String, String>),
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

/// Visibility of files in S3.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum S3Visibility {
    /// Allows anyone to see the file.
    Public,

    /// Only logged in users will be able to see the file.
    Private,

    /// Visibled to logged in users, shared with a pre-signed URL.
    Signed,

    /// Hides the file from everyone.
    None,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum CloudProvider {
    EC2,
    Docker,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionCall {
    pub func: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vars: Option<HashMap<String, ParamValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_secs: Option<u64>,
}

/// Parameters describing how to extract files from a gzipped tarball.
#[derive(Debug, Serialize, Deserialize)]
pub struct ArchiveTargzExtractParams {
    /// Path to tarball to extract.
    pub path: String,
    /// Path of directory to extract files to.
    pub destination: String,
    /// A list of filename globs to exclude.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_files: Option<Vec<String>>,
}

/// Parameters describing how to create a gzipped tarball.
#[derive(Debug, Serialize, Deserialize)]
pub struct ArchiveTargzPackParams {
    /// The tgz files that will be created.
    pub target: String,
    /// The directory to compress.
    pub source_dir: String,
    /// A list of filename globs to include.
    pub include: Vec<String>,
    /// A list of filename globs to exclude.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_files: Option<Vec<String>>,
}

/// Parameters describing how to attach artifacts to a task.
#[derive(Debug, Serialize, Deserialize)]
pub struct AttachArtifactsParams {
    /// An array of gitignore file globs to attach.
    pub files: Vec<String>,
    /// Path to start process the files, relative to the working directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

/// Parameters describing how to attach Evergreen test results format to task.
#[derive(Debug, Serialize, Deserialize)]
pub struct AttachResultsParams {
    /// Path to a json file to parse and upload.
    pub file_location: String,
}

/// Parameters describing how to attach XUnit test results format to task.
#[derive(Debug, Serialize, Deserialize)]
pub struct AttachXUnitResultsParams {
    /// Path to a xunit file to parse and upload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,

    /// List of paths to a xunit file to parse and upload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
}

/// Parameters describing how to update task expansions at runtime.
#[derive(Debug, Serialize, Deserialize)]
pub struct ExpansionsUpdateParams {
    /// key-value pairs for updating the task's expansions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updates: Option<Vec<KeyValueParam>>,

    /// Path to yaml file containing expansion updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,

    /// Do not error if the file is missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_missing_file: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,
}

/// Parameters describing how to write task's expansions to a file.
#[derive(Debug, Serialize, Deserialize)]
pub struct ExpansionsWriteParams {
    /// Path to file to write expansions to.
    pub file: String,

    /// Include redacted project variable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redacted: Option<bool>,
}

/// Parameters describing how to generate dynamic tasks.
#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateTasksParams {
    /// List of json files to generate tasks from.
    pub files: Vec<String>,
}

/// Parameters describing how to clone tracked project and apply revision associated with task.
#[derive(Debug, Serialize, Deserialize)]
pub struct GitGetProjectParams {
    /// Directory to clone repository into.
    pub directory: String,

    /// Token to use to clone instead of ssh key on host.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,

    /// Map of revisions to use for any modules.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revisions: Option<HashMap<String, String>>,
}

/// Parameters describing how to parse gotest results and attach them to the task.
#[derive(Debug, Serialize, Deserialize)]
pub struct GotestParseFilesParams {
    /// List of globs to parse and attach.
    pub files: Vec<String>,
}

/// Parameters describing how to start a new host from a task.
#[derive(Debug, Serialize, Deserialize)]
pub struct HostCreateParams {
    /// Evergreen distro to start.
    pub distro: String,

    /// Cloud Provider for host.
    pub provider: CloudProvider,

    /// List of security groups to set.
    pub security_group_ids: Vec<String>,
}

/// Parameters describing how to get information about hosts previously created.
#[derive(Debug, Serialize, Deserialize)]
pub struct HostListParams {
    /// If `wait` is set, the number of hosts to wait to be running before returning.
    pub num_hosts: u64,
    /// Path of file to write host info to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Time to wait for `num_hosts` to be running.
    pub timeout_seconds: u64,
    /// If true, wait for `num_hosts` to be running.
    pub wait: bool,
    /// If true, do not log host info to the task logs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent: Option<bool>,
}

/// Parameters describing how to save json-formatted task data.
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonSendParams {
    /// Json file to save.
    pub file: String,

    /// Name of the file you're saving.
    pub name: String,
}

/// Parameters describing how to increment a key val.
#[derive(Debug, Serialize, Deserialize)]
pub struct KeyValIncParams {
    pub destination: String,
    pub key: String,
}

/// Parameters describing how to send perf results to cedar.
#[derive(Debug, Serialize, Deserialize)]
pub struct PerfSendParams {
    /// Json or yaml file containing test results.
    pub file: String,

    /// AWS key to upload file with.
    pub aws_key: String,

    /// AWS secret to upload file with.
    pub aws_secret: String,

    /// S3 bucket to upload to.
    pub bucket: String,

    /// Prefix within the S3 bucket.
    pub prefix: String,

    /// AWS region of the bucket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

/// Parameters describing how to download a file from S3.
#[derive(Debug, Serialize, Deserialize)]
pub struct S3GetParams {
    /// Local file to save.
    pub local_file: Option<String>,

    /// Local directory to save to.
    pub extract_to: Option<String>,

    /// S3 Path to get file from.
    pub remote_file: String,

    /// AWS key to use to download file.
    pub aws_key: String,

    /// AWS secret to use to download file.
    pub aws_secret: String,

    /// S3 bucket to upload to.
    pub bucket: String,

    // List of build variants to run command for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_variants: Option<Vec<String>>,
}

/// Parameters describing how to upload a file from S3.
#[derive(Debug, Serialize, Deserialize)]
pub struct S3PutParams {
    /// Local file to upload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_file: Option<String>,

    /// List of globs to indicate files to upload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_files_include_filter: Option<Vec<String>>,

    /// Path to where to start looking for `local_files_include_filter`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_files_include_filter_prefix: Option<String>,

    /// S3 Path to upload to.
    pub remote_file: String,

    /// AWS key to use to download file.
    pub aws_key: String,

    /// AWS secret to use to download file.
    pub aws_secret: String,

    /// S3 bucket to upload to.
    pub bucket: String,

    /// Permission string to upload with.
    pub permissions: String,

    /// The MIME type of the file.
    pub content_type: String,

    /// Display string for file in the Evergreen UI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// If true, do not fail if file is not found.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,

    // AWS region for this bucket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    // AWS visibility of uploaded file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<S3Visibility>,
}

/// Parameters describing how to copy an S3 file.
#[derive(Debug, Serialize, Deserialize)]
pub struct S3CopyParams {
    /// S3 Files to copy.
    pub s3_copy_files: Vec<S3CopyFile>,

    /// AWS key to use to download file.
    pub aws_key: String,

    /// AWS secret to use to download file.
    pub aws_secret: String,
}

/// Parameters describing how to run a shell script.
#[derive(Debug, Serialize, Deserialize)]
pub struct ShellExecParams {
    /// Script to run.
    pub script: String,

    /// Directory to execute shell script in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<String>,

    /// Map of environment variables and their values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,

    /// If true, add all expansions to shell's env.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_expansions_to_env: Option<bool>,

    /// Specify 1 or more expansions to include in the shell's env.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_expansions_in_env: Option<Vec<String>>,

    /// If true, do not wait for script to exit before running next command.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<bool>,

    /// If true, does not log any shell output during execution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent: Option<bool>,

    /// If true, causes command to be marked as success regardless of script's exit code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continue_on_err: Option<bool>,

    /// If true, scripts output will be written to task's system logs instead of test logs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_log: Option<bool>,

    /// Shell to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shell: Option<String>,

    /// If true, discard output sent to stdout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_standard_out: Option<bool>,

    /// If true, discard output sent to stderr.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_standard_error: Option<bool>,

    /// If true, send stderr to stdout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_standard_error_to_output: Option<bool>,
}

/// Parameters describing how to run a binary file.
#[derive(Debug, Serialize, Deserialize)]
pub struct SubprocessExecParams {
    /// Binary to run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary: Option<String>,

    /// Arguments to pass to binary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,

    /// Command String.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,

    /// Directory to execute shell script in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<String>,

    /// Map of environment variables and their values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,

    /// If true, add all expansions to shell's env.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_expansions_to_env: Option<bool>,

    /// Specify 1 or more expansions to include in the shell's env.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_expansions_in_env: Option<Vec<String>>,

    /// If true, do not wait for script to exit before running next command.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<bool>,

    /// If true, does not log any shell output during execution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent: Option<bool>,

    /// If true, causes command to be marked as success regardless of script's exit code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continue_on_err: Option<bool>,

    /// If true, scripts output will be written to task's system logs instead of test logs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_log: Option<bool>,

    /// Shell to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shell: Option<String>,

    /// If true, discard output sent to stdout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_standard_out: Option<bool>,

    /// If true, discard output sent to stderr.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_standard_error: Option<bool>,

    /// If true, send stderr to stdout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_standard_error_to_output: Option<bool>,

    /// List of paths to prepend to PATH.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_to_path: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TimeoutValue {
    Int(u64),
    Expansion(String),
}

/// Parameters describing how to set timeouts for the current task.
#[derive(Debug, Serialize, Deserialize)]
pub struct TimeoutUpdateParams {
    /// Set the maximum time a task can run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec_timeout_secs: Option<TimeoutValue>,

    /// Set the maximum time that can elapse with no output to stdout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_secs: Option<TimeoutValue>,
}

/// Built-in Evergreen Commands.
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "command", content = "params")]
pub enum CommandName {
    /// Extract files from a a gzipped tarball.
    #[serde(alias = "archive.targz_extract")]
    ArchiveTargzExtract(ArchiveTargzExtractParams),

    /// Create a tar-gzipped file.
    #[serde(alias = "archive.targz_pack")]
    ArchiveTargzPack(ArchiveTargzPackParams),

    #[serde(alias = "archive.auto_extract")]
    ArchiveAutoExtract,

    /// Upload files to be include in the "Files" section of a task.
    #[serde(alias = "attach.artifacts")]
    AttachArtifacts(AttachArtifactsParams),

    /// Parse test results in Evergreen's JSON test format and attach to task.
    #[serde(alias = "attach.results")]
    AttachResults(AttachResultsParams),

    /// Parse test results in XUnit format and attach to task.
    #[serde(alias = "attach.xunit_results")]
    AttachXUnitResults(AttachXUnitResultsParams),

    /// Update the task's expansions at runtime.
    #[serde(alias = "expansions.update")]
    ExpansionsUpdate(ExpansionsUpdateParams),

    /// Write the task's expansions to a file.
    #[serde(alias = "expansions.write")]
    ExpansionsWrite(ExpansionsWriteParams),

    /// Dynamically generate tasks from a provided json file.
    #[serde(alias = "generate.tasks")]
    GenerateTasks(GenerateTasksParams),

    /// Clone the tracked project and apply revision associated with task.
    #[serde(alias = "git.get_project")]
    GitGetProject(GitGetProjectParams),

    /// Parse gotest results and attach them to the task.
    #[serde(alias = "gotest.parse_files")]
    GotestParseFiles(GotestParseFilesParams),

    /// Start a new evergreen host.
    #[serde(alias = "host.create")]
    HostCreate(HostCreateParams),

    /// Get information about hosts create with 'hosts.create'.
    #[serde(alias = "host.list")]
    HostList(HostListParams),

    /// Save json-formatted task data to the task.
    #[serde(alias = "json.send")]
    JsonSend(JsonSendParams),

    #[serde(alias = "keyval.inc")]
    KeyValInc(KeyValIncParams),

    /// Update project expansions with the manifest.
    #[serde(alias = "manifest.load")]
    ManifestLoad,

    /// Send performance test data to Cedar.
    #[serde(alias = "perf.send")]
    PerfSend(PerfSendParams),

    /// Download a file from S3.
    #[serde(alias = "s3.get")]
    S3Get(S3GetParams),

    /// Upload a file to S3.
    #[serde(alias = "s3.put")]
    S3Put(S3PutParams),

    /// Copies a file from one S3 location to another.
    #[serde(alias = "s3Copy.copy")]
    S3Copy(S3CopyParams),

    /// Execute the provided shell script.
    #[serde(alias = "shell.exec")]
    ShellExec(ShellExecParams),

    /// Execute the specified binary.
    #[serde(alias = "subprocess.exec")]
    SubprocessExec(SubprocessExecParams),

    #[serde(alias = "subprocess.scripting")]
    SubprocessScripting,

    /// Set the timeouts for the current task.
    #[serde(alias = "timeout.update")]
    TimeoutUpdate(TimeoutUpdateParams),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuiltInCommand {
    #[serde(flatten)]
    pub command: CommandName,
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

pub fn function_call_with_params(name: &str, vars: HashMap<String, ParamValue>) -> Command {
    Command::Function(FunctionCall {
        func: String::from(name),
        vars: Some(vars),
        timeout_secs: None,
    })
}
