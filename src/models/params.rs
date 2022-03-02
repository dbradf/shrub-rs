use core::fmt;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

/// AWS S3 Location description.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct S3Location {
    /// S3 bucket.
    pub bucket: String,
    /// Path within S3 bucket.
    pub path: String,
}

/// Description of how to copy an AWS S3 file.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct KeyValueParam {
    /// Key of Key-Value pair.
    pub key: String,
    /// Value of Key-Value pair.
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ParamValue {
    Bool(bool),
    String(String),
    Number(u64),
    Float(f64),
    List(Vec<String>),
    Map(HashMap<String, String>),
    KeyValueList(Vec<KeyValueParam>),
    S3CopyList(Vec<S3CopyFile>),
}

impl Display for ParamValue {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use ParamValue::*;
        match self {
            Bool(b) => write!(f, "{}", b),
            String(s) => write!(f, "{}", s),
            Number(n) => write!(f, "{}", n),
            Float(d) => write!(f, "{}", d),
            List(l) => write!(f, "{}", l.join(", ")),
            Map(_) => write!(f, "map"),
            KeyValueList(_) => write!(f, "kvs"),
            S3CopyList(_) => write!(f, "s3"),
        }
    }
}

impl From<bool> for ParamValue {
    fn from(item: bool) -> ParamValue {
        ParamValue::Bool(item)
    }
}

impl From<&str> for ParamValue {
    fn from(item: &str) -> ParamValue {
        ParamValue::String(item.to_string())
    }
}

impl From<u64> for ParamValue {
    fn from(item: u64) -> ParamValue {
        ParamValue::Number(item)
    }
}

impl From<f64> for ParamValue {
    fn from(item: f64) -> ParamValue {
        ParamValue::Float(item)
    }
}
