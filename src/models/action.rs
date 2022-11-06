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
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{data_schema::DataSchema, form::Form};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Action<'a> {
    pub forms: Vec<Form<'a>>,
    #[serde(rename = "@type")]
    pub json_ld_type: Option<Vec<&'a str>>,
    pub title: Option<&'a str>,
    pub titles: Option<HashMap<&'a str, &'a str>>,
    pub description: Option<&'a str>,
    pub descriptions: Option<HashMap<&'a str, &'a str>>,
    pub input: Option<DataSchema<'a>>,
    pub output: Option<DataSchema<'a>>,
    pub safe: Option<bool>,
    pub idempotent: Option<bool>,
    pub synchronous: Option<bool>,
}

impl<'a> Action<'a> {
    pub fn builder(forms: Vec<Form>) -> ActionBuilder {
        ActionBuilder::new(forms)
    }
}

#[derive(Debug)]

pub struct ActionBuilder<'a> {
    pub forms: Vec<Form<'a>>,
    pub json_ld_type: Option<Vec<&'a str>>,
    pub title: Option<&'a str>,
    pub titles: Option<HashMap<&'a str, &'a str>>,
    pub description: Option<&'a str>,
    pub descriptions: Option<HashMap<&'a str, &'a str>>,
    pub input: Option<DataSchema<'a>>,
    pub output: Option<DataSchema<'a>>,
    pub safe: Option<bool>,
    pub idempotent: Option<bool>,
    pub synchronous: Option<bool>,
}

impl<'a> ActionBuilder<'a> {
    pub fn new(forms: Vec<Form<'a>>) -> Self {
        ActionBuilder {
            forms,
            json_ld_type: None,
            title: None,
            titles: None,
            description: None,
            descriptions: None,
            input: None,
            output: None,
            safe: None,
            idempotent: None,
            synchronous: None,
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

    pub fn input(mut self, input: DataSchema<'a>) -> Self {
        self.input = Some(input);
        self
    }

    pub fn output(mut self, output: DataSchema<'a>) -> Self {
        self.output = Some(output);
        self
    }

    pub fn safe(mut self, safe: bool) -> Self {
        self.safe = Some(safe);
        self
    }

    pub fn idempotent(mut self, idempotent: bool) -> Self {
        self.idempotent = Some(idempotent);
        self
    }

    pub fn synchronous(mut self, synchronous: bool) -> Self {
        self.synchronous = Some(synchronous);
        self
    }

    pub fn build(self) -> Action<'a> {
        Action {
            forms: self.forms,
            json_ld_type: self.json_ld_type,
            title: self.title,
            titles: self.titles,
            description: self.description,
            descriptions: self.descriptions,
            input: self.input,
            output: self.output,
            safe: self.safe,
            idempotent: self.idempotent,
            synchronous: self.synchronous,
        }
    }
}
