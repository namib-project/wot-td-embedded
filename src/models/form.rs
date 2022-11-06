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

use super::{
    additional_expected_response::AdditionalExpectedResponse, expected_response::ExpectedResponse,
};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Form<'a> {
    pub href: &'a str,
    pub content_type: Option<&'a str>,
    pub content_coding: Option<&'a str>,
    pub security: Option<Vec<&'a str>>,
    pub scopes: Option<Vec<&'a str>>,
    pub response: Option<ExpectedResponse>,
    pub additional_responses: Option<Vec<AdditionalExpectedResponse>>,
    pub subprotocol: Option<&'a str>,
    pub op: Option<Vec<OperationType>>,
}

impl<'a> Form<'a> {
    pub fn builder(href: &'a str) -> FormBuilder {
        FormBuilder::new(href)
    }
}

#[derive(Debug, Default)]
pub struct FormBuilder<'a> {
    pub href: &'a str,
    pub content_type: Option<&'a str>,
    pub content_coding: Option<&'a str>,
    pub security: Option<Vec<&'a str>>,
    pub scopes: Option<Vec<&'a str>>,
    pub response: Option<ExpectedResponse>,
    pub additional_responses: Option<Vec<AdditionalExpectedResponse>>,
    pub subprotocol: Option<&'a str>,
    pub op: Option<Vec<OperationType>>,
}

impl<'a> FormBuilder<'a> {
    pub fn new(href: &'a str) -> Self {
        FormBuilder {
            href,
            ..Default::default()
        }
    }

    pub fn content_type(mut self, content_type: &'a str) -> Self {
        self.content_type = Some(content_type);
        self
    }

    pub fn content_coding(mut self, content_coding: &'a str) -> Self {
        self.content_coding = Some(content_coding);
        self
    }

    pub fn security(mut self, security: Vec<&'a str>) -> Self {
        self.security = Some(security);
        self
    }

    pub fn scopes(mut self, scopes: Vec<&'a str>) -> Self {
        self.scopes = Some(scopes);
        self
    }

    pub fn response(mut self, response: ExpectedResponse) -> Self {
        self.response = Some(response);
        self
    }

    pub fn additional_responses(
        mut self,
        additional_responses: Vec<AdditionalExpectedResponse>,
    ) -> FormBuilder<'a> {
        self.additional_responses = Some(additional_responses);
        self
    }

    pub fn subprotocol(mut self, subprotocol: &'a str) -> Self {
        self.subprotocol = Some(subprotocol);
        self
    }

    pub fn op(mut self, op: Vec<OperationType>) -> Self {
        self.op = Some(op);
        self
    }

    pub fn build(self) -> Form<'a> {
        Form {
            href: self.href,
            content_type: self.content_type,
            content_coding: self.content_coding,
            security: self.security,
            scopes: self.scopes,
            response: self.response,
            additional_responses: self.additional_responses,
            subprotocol: self.subprotocol,
            op: self.op,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
