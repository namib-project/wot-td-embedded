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

use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::data_structures::array::Array;

#[skip_serializing_none]
#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Form<'a> {
    pub href: &'a str,
    pub content_type: Option<&'a str>,
    pub content_coding: Option<&'a str>,
    pub op: Option<Array<'a, OperationType>>,
    pub security: Option<Array<'a, &'a str>>,
    pub scopes: Option<Array<'a, &'a str>>,
    pub subprotocol: Option<&'a str>,
    // TODO: Add response
    // TODO: Add additionalResponses
}

#[derive(Debug, Default)]
pub struct FormBuilder<'a> {
    pub href: &'a str,
    pub content_type: Option<&'a str>,
    pub content_coding: Option<&'a str>,
    pub op: Option<Array<'a, OperationType>>,
    pub security: Option<Array<'a, &'a str>>,
    pub scopes: Option<Array<'a, &'a str>>,
    pub subprotocol: Option<&'a str>,
}

impl<'a> FormBuilder<'a> {
    pub fn new(href: &'a str) -> FormBuilder<'a> {
        FormBuilder {
            href,
            ..Default::default()
        }
    }

    pub fn op(mut self, op: Array<'a, OperationType>) -> FormBuilder<'a> {
        self.op = Some(op);
        self
    }

    pub fn build(self) -> Form<'a> {
        Form {
            href: self.href,
            content_type: self.content_type,
            content_coding: self.content_coding,
            op: self.op,
            security: self.security,
            scopes: self.scopes,
            subprotocol: self.subprotocol,
        }
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum OperationType {
    Readproperty,
    Writeproperty,
    Observeproperty,
    Unobserveproperty,
    Invokeaction,
    Queryaction,
    Cancelaction,
    Subscribeevent,
    Unsubscribeevent,
    Readallproperties,
    Writeallproperties,
    Readmultipleproperties,
    Writemultipleproperties,
    Observeallproperties,
    Unobserveallproperties,
    Subscribeallevents,
    Unsubscribeallevents,
    Queryallactions,
}
