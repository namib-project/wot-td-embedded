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
use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::data_structures::Map;

use super::{data_schema::DataSchema, form::Form};

#[skip_serializing_none]
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Event<'a> {
    pub forms: Vec<Form<'a>>,
    pub json_ld_type: Option<Vec<&'a str>>,
    pub title: Option<&'a str>,
    pub titles: Option<Map<'a, &'a str>>,
    pub description: Option<&'a str>,
    pub descriptions: Option<Map<'a, &'a str>>,
    pub subscription: Option<DataSchema<'a>>,
    pub data: Option<DataSchema<'a>>,
    pub data_response: Option<DataSchema<'a>>,
    pub cancellation: Option<DataSchema<'a>>,
}

impl<'a> Event<'a> {
    pub fn builder(forms: Vec<Form<'a>>) -> EventBuilder<'a> {
        EventBuilder::new(forms)
    }
}

#[derive(Debug)]
pub struct EventBuilder<'a> {
    pub forms: Vec<Form<'a>>,
    pub json_ld_type: Option<Vec<&'a str>>,
    pub title: Option<&'a str>,
    pub titles: Option<Map<'a, &'a str>>,
    pub description: Option<&'a str>,
    pub descriptions: Option<Map<'a, &'a str>>,
    pub subscription: Option<DataSchema<'a>>,
    pub data: Option<DataSchema<'a>>,
    pub data_response: Option<DataSchema<'a>>,
    pub cancellation: Option<DataSchema<'a>>,
}

impl<'a> EventBuilder<'a> {
    pub fn new(forms: Vec<Form<'a>>) -> EventBuilder<'a> {
        EventBuilder {
            forms,
            json_ld_type: None,
            title: None,
            titles: None,
            description: None,
            descriptions: None,
            subscription: None,
            data: None,
            data_response: None,
            cancellation: None,
        }
    }

    // TODO: Add missing builder methods

    pub fn subscription(mut self, subscription: DataSchema<'a>) -> EventBuilder<'a> {
        self.subscription = Some(subscription);
        self
    }

    pub fn data(mut self, data: DataSchema<'a>) -> EventBuilder<'a> {
        self.data = Some(data);
        self
    }

    pub fn data_response(mut self, data_response: DataSchema<'a>) -> EventBuilder<'a> {
        self.data_response = Some(data_response);
        self
    }

    pub fn cancellation(mut self, cancellation: DataSchema<'a>) -> EventBuilder<'a> {
        self.cancellation = Some(cancellation);
        self
    }

    pub fn build(self) -> Event<'a> {
        Event {
            forms: self.forms,
            json_ld_type: self.json_ld_type,
            title: self.title,
            titles: self.titles,
            description: self.description,
            descriptions: self.descriptions,
            subscription: self.subscription,
            data: self.data,
            data_response: self.data_response,
            cancellation: self.cancellation,
        }
    }
}
