use crate::models::params::{KeyValueParam, S3CopyFile};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Describe how task failures should be indicated.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum EvgCommandType {
    /// Failures should indicate a "test" failure.
    Test,
    /// Failures should indicate a "system" failure.
    System,
    /// Failures should indicate a "setup" failure.
    Setup,
}

/// Visibility of files in S3.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum S3Visibility {
    /// Allows anyone to see the file.
    Public,

    /// Only logged in users will be able to see the file.
    Private,

    /// Visible to logged in users, shared with a pre-signed URL.
    Signed,

    /// Hides the file from everyone.
    None,
}

/// Describe which cloud provider should be used.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum CloudProvider {
    /// Use Amazon EC2.
    EC2,
    /// Use docker.
    Docker,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ScriptingHarness {
    Python,
    Python2,
    Golang,
    Roswell,
}

/// Parameters describing how to extract files from a gzipped tarball.
#[derive(Debug, Serialize, Deserialize, Clone)]
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
#[derive(Debug, Serialize, Deserialize, Clone)]
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
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AttachArtifactsParams {
    /// An array of gitignore file globs to attach.
    pub files: Vec<String>,
    /// Path to start process the files, relative to the working directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

/// Parameters describing how to attach Evergreen test results format to task.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AttachResultsParams {
    /// Path to a json file to parse and upload.
    pub file_location: String,
}

/// Parameters describing how to attach XUnit test results format to task.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AttachXUnitResultsParams {
    /// Path to a xunit file to parse and upload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,

    /// List of paths to a xunit file to parse and upload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
}

/// Parameters describing how to update task expansions at runtime.
#[derive(Debug, Serialize, Deserialize, Clone)]
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
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExpansionsWriteParams {
    /// Path to file to write expansions to.
    pub file: String,

    /// Include redacted landscape variable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redacted: Option<bool>,
}

/// Parameters describing how to generate dynamic tasks.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenerateTasksParams {
    /// List of json files to generate tasks from.
    pub files: Vec<String>,
}

/// Parameters describing how to clone tracked landscape and apply revision associated with task.
#[derive(Debug, Serialize, Deserialize, Clone)]
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
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GotestParseFilesParams {
    /// List of globs to parse and attach.
    pub files: Vec<String>,
}

/// Parameters describing how to start a new host from a task.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HostCreateParams {
    /// Evergreen distro to start.
    pub distro: String,

    /// Cloud Provider for host.
    pub provider: CloudProvider,

    /// List of security groups to set.
    pub security_group_ids: Vec<String>,
}

/// Parameters describing how to get information about hosts previously created.
#[derive(Debug, Serialize, Deserialize, Clone)]
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
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JsonSendParams {
    /// Json file to save.
    pub file: String,

    /// Name of the file you're saving.
    pub name: String,
}

/// Parameters describing how to increment a key val.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeyValIncParams {
    pub destination: String,
    pub key: String,
}

/// Parameters describing how to send perf results to cedar.
#[derive(Debug, Serialize, Deserialize, Clone)]
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
#[derive(Debug, Serialize, Deserialize, Clone)]
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
#[derive(Debug, Serialize, Deserialize, Clone)]
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
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S3CopyParams {
    /// S3 Files to copy.
    pub s3_copy_files: Vec<S3CopyFile>,

    /// AWS key to use to download file.
    pub aws_key: String,

    /// AWS secret to use to download file.
    pub aws_secret: String,
}

/// Parameters describing how to run a shell script.
#[derive(Debug, Serialize, Deserialize, Clone)]
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
#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum TimeoutValue {
    Int(u64),
    Expansion(String),
}

/// Parameters describing how to set timeouts for the current task.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TimeoutUpdateParams {
    /// Set the maximum time a task can run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec_timeout_secs: Option<TimeoutValue>,

    /// Set the maximum time that can elapse with no output to stdout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_secs: Option<TimeoutValue>,
}

/// Built-in Evergreen Commands.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "command", content = "params")]
pub enum EvgCommandSpec {
    /// Extract files from a a gzipped tarball.
    #[serde(rename = "archive.targz_extract")]
    ArchiveTargzExtract(ArchiveTargzExtractParams),

    /// Create a tar-gzipped file.
    #[serde(rename = "archive.targz_pack")]
    ArchiveTargzPack(ArchiveTargzPackParams),

    #[serde(rename = "archive.auto_extract")]
    ArchiveAutoExtract,

    /// Upload files to be include in the "Files" section of a task.
    #[serde(rename = "attach.artifacts")]
    AttachArtifacts(AttachArtifactsParams),

    /// Parse test results in Evergreen's JSON test format and attach to task.
    #[serde(rename = "attach.results")]
    AttachResults(AttachResultsParams),

    /// Parse test results in XUnit format and attach to task.
    #[serde(rename = "attach.xunit_results")]
    AttachXUnitResults(AttachXUnitResultsParams),

    /// Update the task's expansions at runtime.
    #[serde(rename = "expansions.update")]
    ExpansionsUpdate(ExpansionsUpdateParams),

    /// Write the task's expansions to a file.
    #[serde(rename = "expansions.write")]
    ExpansionsWrite(ExpansionsWriteParams),

    /// Dynamically generate tasks from a provided json file.
    #[serde(rename = "generate.tasks")]
    GenerateTasks(GenerateTasksParams),

    /// Clone the tracked landscape and apply revision associated with task.
    #[serde(rename = "git.get_project")]
    GitGetProject(GitGetProjectParams),

    /// Parse gotest results and attach them to the task.
    #[serde(rename = "gotest.parse_files")]
    GotestParseFiles(GotestParseFilesParams),

    /// Start a new evergreen host.
    #[serde(rename = "host.create")]
    HostCreate(HostCreateParams),

    /// Get information about hosts create with 'hosts.create'.
    #[serde(rename = "host.list")]
    HostList(HostListParams),

    /// Save json-formatted task data to the task.
    #[serde(rename = "json.send")]
    JsonSend(JsonSendParams),

    #[serde(rename = "keyval.inc")]
    KeyValInc(KeyValIncParams),

    /// Update landscape expansions with the manifest.
    #[serde(rename = "manifest.load")]
    ManifestLoad,

    /// Send performance test data to Cedar.
    #[serde(rename = "perf.send")]
    PerfSend(PerfSendParams),

    /// Download a file from S3.
    #[serde(rename = "s3.get")]
    S3Get(S3GetParams),

    /// Upload a file to S3.
    #[serde(rename = "s3.put")]
    S3Put(S3PutParams),

    /// Copies a file from one S3 location to another.
    #[serde(rename = "s3Copy.copy")]
    S3Copy(S3CopyParams),

    /// Execute the provided shell script.
    #[serde(rename = "shell.exec")]
    ShellExec(ShellExecParams),

    /// Execute the specified binary.
    #[serde(rename = "subprocess.exec")]
    SubprocessExec(SubprocessExecParams),

    #[serde(rename = "subprocess.scripting")]
    SubprocessScripting,

    /// Set the timeouts for the current task.
    #[serde(rename = "timeout.update")]
    TimeoutUpdate(TimeoutUpdateParams),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BuiltInCommand {
    /// Description the built-in command to run.
    #[serde(flatten)]
    pub command: EvgCommandSpec,

    /// How command status should be indicated.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub command_type: Option<EvgCommandType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    params_yaml: Option<String>,
}
