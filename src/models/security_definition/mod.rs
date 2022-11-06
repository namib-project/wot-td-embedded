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

pub mod ace;
pub mod api_key;
pub mod basic;
pub mod bearer;
pub mod combo;
pub mod digest;
pub mod oauth2;
pub mod psk;

use alloc::vec::Vec;
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

use self::{
    ace::AceSecurityScheme, api_key::ApiKeySecurityScheme, basic::BasicSecurityScheme,
    bearer::BearerSecurityScheme, combo::ComboSecurityScheme, digest::DigestSecurityScheme,
    oauth2::Oauth2SecurityScheme, psk::PskSecurityScheme,
};

use serde_with::skip_serializing_none;
// TODO: Add possibility to define additional fields
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityScheme<'a> {
    pub json_ld_type: Option<Vec<&'a str>>,
    pub description: Option<&'a str>,
    pub descriptions: Option<HashMap<&'a str, &'a str>>,
    pub proxy: Option<&'a str>,
    #[serde(flatten)]
    pub scheme: SecuritySchemeType<'a>,
}

impl<'a> SecurityScheme<'a> {
    pub fn builder(scheme: SecuritySchemeType) -> SecuritySchemeBuilder {
        SecuritySchemeBuilder::new(scheme)
    }
}

#[derive(Debug)]
pub struct SecuritySchemeBuilder<'a> {
    pub json_ld_type: Option<Vec<&'a str>>,
    pub description: Option<&'a str>,
    pub descriptions: Option<HashMap<&'a str, &'a str>>,
    pub proxy: Option<&'a str>,
    pub scheme: SecuritySchemeType<'a>,
}

impl<'a> SecuritySchemeBuilder<'a> {
    pub fn new(scheme: SecuritySchemeType<'a>) -> Self {
        Self {
            scheme,
            json_ld_type: None,
            description: None,
            descriptions: None,
            proxy: None,
        }
    }

    pub fn json_ld_type(mut self, json_ld_type: Vec<&'a str>) -> Self {
        self.json_ld_type = Some(json_ld_type);
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

    pub fn proxy(mut self, proxy: &'a str) -> Self {
        self.proxy = Some(proxy);
        self
    }

    pub fn build(self) -> SecurityScheme<'a> {
        SecurityScheme {
            json_ld_type: self.json_ld_type,
            description: self.description,
            descriptions: self.descriptions,
            proxy: self.proxy,
            scheme: self.scheme,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "scheme")]
#[serde(rename_all = "lowercase")]
pub enum SecuritySchemeType<'a> {
    Nosec,
    Auto,
    Combo(ComboSecurityScheme),
    Basic(BasicSecurityScheme),
    #[serde(borrow)]
    Digest(DigestSecurityScheme<'a>),
    #[serde(borrow)]
    Apikey(ApiKeySecurityScheme<'a>),
    #[serde(borrow)]
    Bearer(BearerSecurityScheme<'a>),
    #[serde(borrow)]
    Psk(PskSecurityScheme<'a>),
    #[serde(borrow)]
    Oauth2(Oauth2SecurityScheme<'a>),
    #[serde(borrow)]
    Ace(AceSecurityScheme<'a>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum In {
    Header,
    Query,
    Body,
    Cookie,
    Auto,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum QoP {
    Auth,
    AuthInt,
}
