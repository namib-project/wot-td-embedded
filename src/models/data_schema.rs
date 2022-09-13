use heapless::FnvIndexMap;
use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::data_structures::{array::ArrayEntry, map::MapEntry};

#[skip_serializing_none]
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataSchema<'a, const TITLES: usize = 0, const DESCRIPTIONS: usize = 0> {
    #[serde(rename = "@type")]
    pub json_ld_type: Option<ArrayEntry<'a, &'a str>>,
    pub title: Option<&'a str>,
    pub titles: Option<FnvIndexMap<&'a str, &'a str, TITLES>>,
    pub description: Option<&'a str>,
    pub descriptions: Option<FnvIndexMap<&'a str, &'a str, DESCRIPTIONS>>,
    #[serde(rename = "const")]
    pub constant: Option<DataStructure<'a>>,
    pub default: Option<DataStructure<'a>>,
    pub unit: Option<&'a str>,
    #[serde(rename = "oneOf")]
    pub one_of: Option<&'a MapEntry<'a, DataSchema<'a>>>,
    #[serde(rename = "enum")]
    pub enumeration: Option<DataStructure<'a>>,
    #[serde(rename = "readOnly")]
    pub read_only: Option<bool>,
    #[serde(rename = "writeOnly")]
    pub write_only: Option<bool>,
    pub format: Option<&'a str>,
    #[serde(rename = "type")]
    pub data_type: Option<DataType>,
}

#[derive(Serialize, Debug)]
pub enum DataType {
    #[serde(rename = "object")]
    Object,
    #[serde(rename = "array")]
    Array,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "null")]
    Null,
}

#[skip_serializing_none]
#[derive(Serialize, Debug)]
pub enum DataStructure<'a> {
    String(&'a str),
    Integer(i64),
    Float(f64),
    // Object(&'a MapEntry<'a, &'a dyn Serialize>),
    // Array(ArrayEntry<'a, &'a dyn Serialize>),
}
