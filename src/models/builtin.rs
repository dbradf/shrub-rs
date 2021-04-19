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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum TimeoutValue {
    Int(u64),
    Expansion(String),
}

impl From<u64> for TimeoutValue {
    fn from(item: u64) -> TimeoutValue {
        TimeoutValue::Int(item)
    }
}

impl From<&str> for TimeoutValue {
    fn from(item: &str) -> TimeoutValue {
        TimeoutValue::Expansion(item.to_string())
    }
}

#[cfg(test)]
mod timeout_value_tests {
    use super::*;

    #[test]
    fn test_int_to_timeout_value() {
        let timeout = TimeoutValue::from(42);

        assert_eq!(TimeoutValue::Int(42), timeout);
    }

    #[test]
    fn test_string_to_timeout_value() {
        let timeout = TimeoutValue::from("${timeout_expansion}");

        assert_eq!(
            TimeoutValue::Expansion(String::from("${timeout_expansion}")),
            timeout
        );
    }
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

    /// Don't fail if files are not found to attach.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_artifacts_for_spawn: Option<bool>,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum HostScope {
    /// Tear down host when task is finished.
    Task,

    /// Tear down host when build is finished.
    Build,
}

/// Description of an EBS block device.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EbsDevice {
    pub device_name: String,
    pub ebs_iops: u64,
    pub ebs_size: u64,
    pub ebs_snapshot_id: String,
}

/// Docker registry settings.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegistrySettings {
    /// Registry to pull image from.
    pub registry_name: String,

    /// Username for the registry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_username: Option<String>,

    /// Password for the registry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_password: Option<String>,
}

/// Parameters describing how to start a new host from a task.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HostCreateParams {
    /// Name of a file containing all the parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,

    // Agent Params
    /// Number of hosts to start, between 1 and 10 defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_hosts: Option<u16>,

    /// Cloud Provider for host.
    pub provider: CloudProvider,

    /// How many times Evergreen should try to create this host.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<u64>,

    /// When Evergreen will tear down the host.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<HostScope>,

    /// Stop waiting for hosts to be ready when spawning.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_setup_secs: Option<u64>,

    /// Tear down this host after this many seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_teardown_secs: Option<u64>,

    // EC2 Params
    /// EC2 AMI to start.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ami: Option<String>,

    /// AWS access key ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_access_key_id: Option<String>,

    /// AWS secret key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_secret_access_key: Option<String>,

    /// Name of EBS device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,

    /// Evergreen distro to start.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distro: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_block_device: Option<EbsDevice>,

    /// EC2 Instance type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,

    /// Indicates instance should only have IPv6 address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<bool>,

    /// EC2 region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    /// List of security groups to set.
    pub security_group_ids: Vec<String>,

    /// Swap a spot instance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot: Option<bool>,

    /// Subnet ID for the VPC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,

    /// Path to file to load as EC2 user data on boot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userdata_file: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub userdata_command: Option<String>,

    /// Ec2 Key name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,

    // docker settings.
    /// Docker image to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,

    /// Command to run on the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,

    /// make ports available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_ports: Option<bool>,

    /// Information of registry to pull image from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry: Option<RegistrySettings>,

    /// Set to wait for logs in the background.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<bool>,

    /// Time to wait for the container to finish running.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_wait_timeout_secs: Option<u64>,

    /// Check for running container and logs at this interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_frequency_secs: Option<u64>,

    /// Path to write stdout logs from the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdout_file_name: Option<String>,

    /// Path to write stderr logs from the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stderr_file_name: Option<String>,

    /// Map of environment variables to pass to container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_vars: Option<HashMap<String, String>>,
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

/// Parameters commont to SubprocessExec and SubprocessScripting.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubprocessExecutionConfig {
    /// If true, does not log any shell output during execution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent: Option<bool>,

    /// If true, causes command to be marked as success regardless of script's exit code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continue_on_err: Option<bool>,

    /// If true, scripts output will be written to task's system logs instead of test logs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_log: Option<bool>,

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

    /// If true, add all expansions to shell's env.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_expansions_to_env: Option<bool>,

    /// Specify 1 or more expansions to include in the shell's env.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_expansions_in_env: Option<Vec<String>>,
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

    /// If true, do not wait for script to exit before running next command.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<bool>,

    /// Shell to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shell: Option<String>,

    /// Execution configuration.
    #[serde(flatten)]
    pub execution_config: SubprocessExecutionConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScriptingTestOptions {
    /// Name of test
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Any additional argument to the test binary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,

    /// Filter names of tests to run based on this pattern.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_secs: Option<u64>,

    /// The number of times test should be run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u64>,
}

/// Parameters describing how to execute a command insdie a scripting harness.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubprocessScriptingParams {
    /// Scripting harness to use.
    pub harness: ScriptingHarness,

    /// Commandline args as a to run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,

    /// Commandline args as a to run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,

    /// Directory where tets should be run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_dir: Option<String>,

    /// Describes how to tests in test_dir will be executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_options: Option<ScriptingTestOptions>,

    /// Total number of seconds environment should be stored for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_duration_secs: Option<u64>,

    /// Indicates that harness should not be reused between executions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cleanup_harness: Option<bool>,

    /// Lockfile describing dependencies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_file: Option<String>,

    /// List of dependencies to install.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packages: Option<Vec<String>>,

    /// Path to hosting interpreter or binary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub harness_path: Option<String>,

    /// Execution configuration.
    #[serde(flatten)]
    pub execution_config: SubprocessExecutionConfig,
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

    /// Execute a command inside a scripting harness.
    #[serde(rename = "subprocess.scripting")]
    SubprocessScripting(SubprocessScriptingParams),

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
