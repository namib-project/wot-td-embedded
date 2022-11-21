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

use alloc::vec::Vec;
use serde::{ser::SerializeMap, Serialize};

use super::{data_schema::DataSchema, form::Form};

#[derive(Debug)]
pub struct Property<'a> {
    pub forms: Vec<Form<'a>>,
    pub data_schema: DataSchema<'a>,
    pub observable: Option<bool>,
}

impl<'a> Property<'a> {
    pub fn builder(forms: Vec<Form<'a>>, data_schema: DataSchema<'a>) -> PropertyBuilder<'a> {
        PropertyBuilder::new(forms, data_schema)
    }
}

impl<'a> Serialize for Property<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(None)?;

        map.serialize_entry("forms", &self.forms)?;

        if let Some(value) = &self.observable {
            map.serialize_entry("observable", value)?;
        }

        self.data_schema.serialize_to_map::<S>(map)?.end()
    }
}

#[derive(Debug)]
pub struct PropertyBuilder<'a> {
    pub forms: Vec<Form<'a>>,
    pub data_schema: DataSchema<'a>,
    pub observable: Option<bool>,
}

impl<'a> PropertyBuilder<'a> {
    pub fn new(forms: Vec<Form<'a>>, data_schema: DataSchema<'a>) -> PropertyBuilder<'a> {
        PropertyBuilder {
            forms,
            data_schema,
            observable: None,
        }
    }

    pub fn observable(&mut self, observable: bool) -> &PropertyBuilder<'a> {
        self.observable = Some(observable);
        self
    }

    pub fn data_schema(&mut self, data_schema: DataSchema<'a>) -> &PropertyBuilder<'a> {
        self.data_schema = data_schema;
        self
    }

    pub fn build(self) -> Property<'a> {
        Property {
            forms: self.forms,
            data_schema: self.data_schema,
            observable: self.observable,
        }
    }
}
