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

pub mod array_schema;
pub mod integer_schema;
pub mod number_schema;
pub mod object_schema;
pub mod string_schema;

use serde::{ser::SerializeMap, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    constants::JSON_LD_TYPE,
    data_structures::{
        array::{Array, ArrayEntry},
        map::{Map, MapEntry},
    },
    models::serialize_field,
};

use self::{
    array_schema::ArraySchema, integer_schema::IntegerSchema, number_schema::NumberSchema,
    object_schema::ObjectSchema, string_schema::StringSchema,
};

macro_rules! serialize_schema {
    ($schema:expr, $map:expr) => {
        if let Some(schema) = &$schema {
            $map = schema.serialize_to_map::<S>($map)?;
        }
    };
}

#[derive(Debug)]
pub struct DataSchema<'a> {
    pub json_ld_type: Option<Array<'a, &'a str>>,
    pub title: Option<&'a str>,
    pub titles: Option<Map<'a, &'a str>>,
    pub description: Option<&'a str>,
    pub descriptions: Option<Map<'a, &'a str>>,
    pub constant: Option<DataStructure<'a>>,
    pub default: Option<DataStructure<'a>>,
    pub unit: Option<&'a str>,
    pub one_of: Option<Array<'a, &'a DataSchema<'a>>>,
    pub enumeration: Option<Array<'a, DataStructure<'a>>>,
    pub read_only: Option<bool>,
    pub write_only: Option<bool>,
    pub format: Option<&'a str>,
    pub data_type: Option<DataType<'a>>,
    pub additional_fields: Option<Map<'a, DataStructure<'a>>>,
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
    pub json_ld_type: Option<Array<'a, &'a str>>,
    pub title: Option<&'a str>,
    pub titles: Option<Map<'a, &'a str>>,
    pub description: Option<&'a str>,
    pub descriptions: Option<Map<'a, &'a str>>,
    pub constant: Option<DataStructure<'a>>,
    pub default: Option<DataStructure<'a>>,
    pub unit: Option<&'a str>,
    pub one_of: Option<Array<'a, &'a DataSchema<'a>>>,
    pub enumeration: Option<Array<'a, DataStructure<'a>>>,
    pub read_only: Option<bool>,
    pub write_only: Option<bool>,
    pub format: Option<&'a str>,
    pub data_type: Option<DataType<'a>>,
    pub additional_fields: Option<Map<'a, DataStructure<'a>>>,
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

    pub fn titles(mut self, titles: Map<'a, &'a str>) -> Self {
        self.titles = Some(titles);
        self
    }

    pub fn description(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }

    pub fn descriptions(mut self, descriptions: Map<'a, &'a str>) -> Self {
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

    pub fn data_type(mut self, data_type: DataType<'a>) -> Self {
        self.data_type = Some(data_type);
        self
    }

    pub fn additional_fields(mut self, additional_fields: Map<'a, DataStructure<'a>>) -> Self {
        self.additional_fields = Some(additional_fields);
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
            additional_fields: self.additional_fields,
        }
    }
}

impl<'a> DataSchema<'a> {
    pub fn serialize_to_map<S>(&self, mut map: S::SerializeMap) -> Result<S::SerializeMap, S::Error>
    where
        S: serde::Serializer,
    {
        serialize_field::<ArrayEntry<'a, &'a str>, S>(&self.json_ld_type, JSON_LD_TYPE, &mut map)?;
        serialize_field::<&str, S>(&self.title, "title", &mut map)?;
        serialize_field::<MapEntry<'a, &'a str>, S>(&self.titles, "titles", &mut map)?;
        serialize_field::<&str, S>(&self.description, "description", &mut map)?;
        serialize_field::<MapEntry<'a, &'a str>, S>(&self.descriptions, "descriptions", &mut map)?;
        serialize_field::<DataStructure, S>(&self.constant, "constant", &mut map)?;
        serialize_field::<DataStructure, S>(&self.default, "default", &mut map)?;
        serialize_field::<&str, S>(&self.unit, "unit", &mut map)?;
        serialize_field::<ArrayEntry<'a, &'a DataSchema>, S>(&self.one_of, "oneOf", &mut map)?;
        serialize_field::<ArrayEntry<'a, DataStructure>, S>(&self.enumeration, "enum", &mut map)?;
        serialize_field::<bool, S>(&self.read_only, "readOnly", &mut map)?;
        serialize_field::<bool, S>(&self.write_only, "writeOnly", &mut map)?;
        serialize_field::<&str, S>(&self.format, "format", &mut map)?;

        let mut map = map;

        if let Some(data_type) = &self.data_type {
            map.serialize_key("type")?;
            match data_type {
                DataType::Object(schema) => {
                    map.serialize_value("object")?;
                    serialize_schema!(schema, map);
                }
                DataType::Array(schema) => {
                    map.serialize_value("array")?;
                    serialize_schema!(schema, map);
                }
                DataType::String(schema) => {
                    map.serialize_value("string")?;
                    serialize_schema!(schema, map);
                }
                DataType::Number(schema) => {
                    map.serialize_value("number")?;
                    serialize_schema!(schema, map);
                }
                DataType::Integer(schema) => {
                    map.serialize_value("integer")?;
                    serialize_schema!(schema, map);
                }
                DataType::Boolean => {
                    map.serialize_value("boolean")?;
                }
                DataType::Null => {
                    map.serialize_value("null")?;
                }
            }
        }

        if let Some(additional_fields) = &self.additional_fields {
            for map_entry in additional_fields.iter() {
                map.serialize_entry(map_entry.key, map_entry.value)?;
            }
        }

        Ok(map)
    }
}

#[derive(Debug)]
pub enum DataType<'a> {
    Object(Option<ObjectSchema<'a>>),
    Array(Option<ArraySchema<'a>>),
    String(Option<StringSchema<'a>>),
    Number(Option<NumberSchema>),
    Integer(Option<IntegerSchema>),
    Boolean,
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
