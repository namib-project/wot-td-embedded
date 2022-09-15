/*
 * Copyright (c) 2022 The NAMIB Project Developers.
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 *
 * SPDX-License-Identifier: MIT OR Apache-2.0
 */

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
    pub one_of: Option<Array<'a, &'a DataSchema<'a>>>,
    pub enumeration: Option<Array<'a, DataStructure<'a>>>,
    pub read_only: Option<bool>,
    pub write_only: Option<bool>,
    pub format: Option<&'a str>,
    pub data_type: Option<DataType>,
}

impl<'a> DataSchema<'a> {
    pub fn builder() -> DataSchemaBuilder<'a> {
        DataSchemaBuilder {
            ..Default::default()
        }
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

#[derive(Debug, Default)]
pub struct DataSchemaBuilder<'a> {
    // TODO: Add specific data types
    pub json_ld_type: Option<Array<'a, &'a str>>,
    pub title: Option<&'a str>,
    pub titles: Option<&'a Map<'a, &'a str>>,
    pub description: Option<&'a str>,
    pub descriptions: Option<&'a Map<'a, &'a str>>,
    pub constant: Option<DataStructure<'a>>,
    pub default: Option<DataStructure<'a>>,
    pub unit: Option<&'a str>,
    pub one_of: Option<Array<'a, &'a DataSchema<'a>>>,
    pub enumeration: Option<Array<'a, DataStructure<'a>>>,
    pub read_only: Option<bool>,
    pub write_only: Option<bool>,
    pub format: Option<&'a str>,
    pub data_type: Option<DataType>,
}

impl<'a> DataSchemaBuilder<'a> {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn json_ld_type(mut self, json_ld_type: Array<'a, &'a str>) -> Self {
        self.json_ld_type = Some(json_ld_type);
        self
    }

    pub fn title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }

    pub fn titles(mut self, titles: &'a Map<'a, &'a str>) -> Self {
        self.titles = Some(titles);
        self
    }

    pub fn description(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }

    pub fn descriptions(mut self, descriptions: &'a Map<'a, &'a str>) -> Self {
        self.descriptions = Some(descriptions);
        self
    }

    pub fn constant(mut self, constant: DataStructure<'a>) -> Self {
        self.constant = Some(constant);
        self
    }

    pub fn default(mut self, default: DataStructure<'a>) -> Self {
        self.default = Some(default);
        self
    }

    pub fn unit(mut self, unit: &'a str) -> Self {
        self.unit = Some(unit);
        self
    }

    pub fn one_of(mut self, one_of: Array<'a, &'a DataSchema<'a>>) -> Self {
        self.one_of = Some(one_of);
        self
    }

    pub fn enumeration(mut self, enumeration: Array<'a, DataStructure<'a>>) -> Self {
        self.enumeration = Some(enumeration);
        self
    }

    pub fn read_only(mut self, read_only: bool) -> Self {
        self.read_only = Some(read_only);
        self
    }

    pub fn write_only(mut self, write_only: bool) -> Self {
        self.write_only = Some(write_only);
        self
    }

    pub fn format(mut self, format: &'a str) -> Self {
        self.format = Some(format);
        self
    }

    pub fn data_type(mut self, data_type: DataType) -> Self {
        self.data_type = Some(data_type);
        self
    }

    pub fn build(self) -> DataSchema<'a> {
        DataSchema {
            json_ld_type: self.json_ld_type,
            title: self.title,
            titles: self.titles,
            description: self.description,
            descriptions: self.descriptions,
            constant: self.constant,
            default: self.default,
            unit: self.unit,
            one_of: self.one_of,
            enumeration: self.enumeration,
            read_only: self.read_only,
            write_only: self.write_only,
            format: self.format,
            data_type: self.data_type,
        }
    }
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
