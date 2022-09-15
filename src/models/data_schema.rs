use serde::{ser::SerializeMap, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    data_structures::{array::Array, map::Map},
    serialize_field,
};

#[derive(Debug)]
pub struct DataSchema<'a> {
    pub json_ld_type: Option<Array<'a, &'a str>>,
    pub title: Option<&'a str>,
    pub titles: Option<&'a Map<'a, &'a str>>,
    pub description: Option<&'a str>,
    pub descriptions: Option<&'a Map<'a, &'a str>>,
    pub constant: Option<DataStructure<'a>>,
    pub default: Option<DataStructure<'a>>,
    pub unit: Option<&'a str>,
    pub one_of: Option<Map<'a, DataSchema<'a>>>,
    pub enumeration: Option<DataStructure<'a>>,
    pub read_only: Option<bool>,
    pub write_only: Option<bool>,
    pub format: Option<&'a str>,
    pub data_type: Option<DataType>,
}

impl<'a> DataSchema<'a> {
    pub fn serialize_to_map<S>(&self, mut map: S::SerializeMap) -> Result<S::SerializeMap, S::Error>
    where
        S: serde::Serializer,
    {
        serialize_field!("@type", &self.json_ld_type, map);
        serialize_field!("title", &self.title, map);
        serialize_field!("titles", &self.titles, map);
        serialize_field!("description", &self.description, map);
        serialize_field!("descriptions", &self.descriptions, map);
        serialize_field!("constant", &self.constant, map);
        serialize_field!("default", &self.default, map);
        serialize_field!("unit", &self.unit, map);
        serialize_field!("oneOf", &self.one_of, map);
        serialize_field!("enum", &self.enumeration, map);
        serialize_field!("readOnly", &self.read_only, map);
        serialize_field!("writeOnly", &self.write_only, map);
        serialize_field!("format", &self.format, map);
        serialize_field!("type", &self.data_type, map);

        Ok(map)
    }
}

impl<'a> Serialize for DataSchema<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let map = serializer.serialize_map(None)?;

        self.serialize_to_map::<S>(map)?.end()
    }
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
    Null,
    String(&'a str),
    Integer(i64),
    Number(f64),
    Object(Map<'a, &'a DataStructure<'a>>),
    Array(Array<'a, &'a DataStructure<'a>>),
}
