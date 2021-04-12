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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeyValueParam {
    /// Key of Key-Value pair.
    pub key: String,
    /// Value of Key-Value pair.
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
