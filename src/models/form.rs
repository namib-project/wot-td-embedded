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

use crate::data_structures::Array;

use super::{
    additional_expected_response::AdditionalExpectedResponse, expected_response::ExpectedResponse,
};

#[skip_serializing_none]
#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Form<'a> {
    pub href: &'a str,
    pub content_type: Option<&'a str>,
    pub content_coding: Option<&'a str>,
    pub security: Option<Array<&'a str>>,
    pub scopes: Option<Array<&'a str>>,
    pub response: Option<ExpectedResponse<'a>>,
    pub additional_responses: Option<Array<AdditionalExpectedResponse<'a>>>,
    pub subprotocol: Option<&'a str>,
    pub op: Option<Array<OperationType>>,
}

impl<'a> Form<'a> {
    pub fn builder(href: &'a str) -> FormBuilder<'a> {
        FormBuilder::new(href)
    }
}

#[derive(Debug, Default)]
pub struct FormBuilder<'a> {
    pub href: &'a str,
    pub content_type: Option<&'a str>,
    pub content_coding: Option<&'a str>,
    pub security: Option<Array<&'a str>>,
    pub scopes: Option<Array<&'a str>>,
    pub response: Option<ExpectedResponse<'a>>,
    pub additional_responses: Option<Array<AdditionalExpectedResponse<'a>>>,
    pub subprotocol: Option<&'a str>,
    pub op: Option<Array<OperationType>>,
}

impl<'a> FormBuilder<'a> {
    pub fn new(href: &'a str) -> FormBuilder<'a> {
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

    pub fn security(mut self, security: Array<&'a str>) -> FormBuilder<'a> {
        self.security = Some(security);
        self
    }

    pub fn scopes(mut self, scopes: Array<&'a str>) -> FormBuilder<'a> {
        self.scopes = Some(scopes);
        self
    }

    pub fn response(mut self, response: ExpectedResponse<'a>) -> FormBuilder<'a> {
        self.response = Some(response);
        self
    }

    pub fn additional_responses(
        mut self,
        additional_responses: Array<AdditionalExpectedResponse<'a>>,
    ) -> FormBuilder<'a> {
        self.additional_responses = Some(additional_responses);
        self
    }

    pub fn subprotocol(mut self, subprotocol: &'a str) -> Self {
        self.subprotocol = Some(subprotocol);
        self
    }

    pub fn op(mut self, op: Array<OperationType>) -> FormBuilder<'a> {
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
