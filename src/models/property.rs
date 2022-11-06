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
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{data_schema::DataSchema, form::Form};

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property<'a> {
    #[serde(flatten, borrow)]
    pub data_schema: DataSchema<'a>,
    pub observable: Option<bool>,
    #[serde(borrow)]
    pub forms: Vec<Form<'a>>,
}

impl<'a> Property<'a> {
    pub fn builder(forms: Vec<Form<'a>>, data_schema: DataSchema<'a>) -> PropertyBuilder<'a> {
        PropertyBuilder::new(forms, data_schema)
    }
}

#[derive(Debug)]
pub struct PropertyBuilder<'a> {
    pub forms: Vec<Form<'a>>,
    pub data_schema: DataSchema<'a>,
    pub observable: Option<bool>,
}

impl<'a> PropertyBuilder<'a> {
    pub fn new(forms: Vec<Form<'a>>, data_schema: DataSchema<'a>) -> Self {
        PropertyBuilder {
            forms,
            data_schema,
            observable: None,
        }
    }

    pub fn observable(mut self, observable: bool) -> Self {
        self.observable = Some(observable);
        self
    }

    pub fn data_schema(mut self, data_schema: DataSchema<'a>) -> Self {
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
