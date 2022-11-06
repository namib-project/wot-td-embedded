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

use alloc::vec::Vec;
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use self::{
    array_schema::ArraySchema, integer_schema::IntegerSchema, number_schema::NumberSchema,
    object_schema::ObjectSchema, string_schema::StringSchema,
};

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSchema<'a> {
    pub json_ld_type: Option<Vec<&'a str>>,
    pub title: Option<&'a str>,
    pub titles: Option<HashMap<&'a str, &'a str>>,
    pub description: Option<&'a str>,
    pub descriptions: Option<HashMap<&'a str, &'a str>>,
    pub constant: Option<DataStructure<'a>>,
    pub default: Option<DataStructure<'a>>,
    pub unit: Option<&'a str>,
    pub one_of: Option<Vec<DataSchema<'a>>>,
    pub enumeration: Option<Vec<DataStructure<'a>>>,
    pub read_only: Option<bool>,
    pub write_only: Option<bool>,
    pub format: Option<&'a str>,
    #[serde(flatten)]
    pub data_type: Option<DataType<'a>>,
    pub additional_fields: Option<HashMap<&'a str, DataStructure<'a>>>,
}

impl<'a> DataSchema<'a> {
    pub fn builder() -> DataSchemaBuilder<'a> {
        DataSchemaBuilder {
            ..Default::default()
        }
    }
}

#[derive(Debug, Default)]
pub struct DataSchemaBuilder<'a> {
    pub json_ld_type: Option<Vec<&'a str>>,
    pub title: Option<&'a str>,
    pub titles: Option<HashMap<&'a str, &'a str>>,
    pub description: Option<&'a str>,
    pub descriptions: Option<HashMap<&'a str, &'a str>>,
    pub constant: Option<DataStructure<'a>>,
    pub default: Option<DataStructure<'a>>,
    pub unit: Option<&'a str>,
    pub one_of: Option<Vec<DataSchema<'a>>>,
    pub enumeration: Option<Vec<DataStructure<'a>>>,
    pub read_only: Option<bool>,
    pub write_only: Option<bool>,
    pub format: Option<&'a str>,

    pub data_type: Option<DataType<'a>>,
    pub additional_fields: Option<HashMap<&'a str, DataStructure<'a>>>,
}

impl<'a> DataSchemaBuilder<'a> {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn json_ld_type(mut self, json_ld_type: Vec<&'a str>) -> Self {
        self.json_ld_type = Some(json_ld_type);
        self
    }

    pub fn title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }

    pub fn titles(mut self, titles: HashMap<&'a str, &'a str>) -> Self {
        self.titles = Some(titles);
        self
    }

    pub fn description(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }

    pub fn descriptions(mut self, descriptions: HashMap<&'a str, &'a str>) -> Self {
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

    pub fn one_of(mut self, one_of: Vec<DataSchema<'a>>) -> Self {
        self.one_of = Some(one_of);
        self
    }

    pub fn enumeration(mut self, enumeration: Vec<DataStructure<'a>>) -> Self {
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

    pub fn additional_fields(
        mut self,
        additional_fields: HashMap<&'a str, DataStructure<'a>>,
    ) -> Self {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]

pub enum DataType<'a> {
    #[serde(borrow)]
    Object(ObjectSchema<'a>),
    #[serde(borrow)]
    Array(ArraySchema<'a>),
    #[serde(borrow)]
    String(StringSchema<'a>),
    Number(NumberSchema),
    Integer(IntegerSchema),
    Boolean,
    Null,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DataStructure<'a> {
    Null,
    String(&'a str),
    Integer(i64),
    Number(f64),
    Object(HashMap<&'a str, DataStructure<'a>>),
    Array(Vec<DataStructure<'a>>),
}
