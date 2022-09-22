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

use serde::{ser::SerializeMap, Serialize};

use crate::{
    constants::JSON_LD_TYPE,
    data_structures::{
        array::{Array, ArrayEntry},
        map::{Map, MapEntry},
    },
    models::serialize_field,
};

use self::{
    ace::AceSecurityScheme, api_key::ApiKeySecurityScheme, basic::BasicSecurityScheme,
    bearer::BearerSecurityScheme, combo::ComboSecurityScheme, combo::ComboVariant,
    digest::DigestSecurityScheme, oauth2::Oauth2SecurityScheme, psk::PskSecurityScheme,
};

// TODO: Add possibility to define additional fields

#[derive(Debug)]
pub struct SecurityScheme<'a> {
    pub json_ld_type: Option<Array<'a, &'a str>>,
    pub description: Option<&'a str>,
    pub descriptions: Option<&'a Map<'a, &'a str>>,
    pub proxy: Option<&'a str>,
    pub scheme: SecuritySchemeType<'a>,
}

impl<'a> SecurityScheme<'a> {
    pub fn builder(scheme: SecuritySchemeType<'a>) -> SecuritySchemeBuilder<'a> {
        SecuritySchemeBuilder::new(scheme)
    }
}

#[derive(Debug)]
pub struct SecuritySchemeBuilder<'a> {
    pub json_ld_type: Option<Array<'a, &'a str>>,
    pub description: Option<&'a str>,
    pub descriptions: Option<&'a Map<'a, &'a str>>,
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

    pub fn json_ld_type(mut self, json_ld_type: Array<'a, &'a str>) -> Self {
        self.json_ld_type = Some(json_ld_type);
        self
    }

    pub fn description(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }

    pub fn descriptions(mut self, descriptions: &'a Map<'a, &'a str>) -> Self {
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

#[derive(Debug)]
pub enum SecuritySchemeType<'a> {
    Nosec,
    Auto,
    Combo(ComboSecurityScheme<'a>),
    Basic(BasicSecurityScheme<'a>),
    Digest(DigestSecurityScheme<'a>),
    Apikey(ApiKeySecurityScheme<'a>),
    Bearer(BearerSecurityScheme<'a>),
    Psk(PskSecurityScheme<'a>),
    Oauth2(Oauth2SecurityScheme<'a>),
    Ace(AceSecurityScheme<'a>),
}

impl<'a> Serialize for SecurityScheme<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(None)?;

        map.serialize_key("scheme")?;

        match &self.scheme {
            SecuritySchemeType::Auto => {
                map.serialize_value("auto")?;
            }
            SecuritySchemeType::Nosec => {
                map.serialize_value("nosec")?;
            }
            SecuritySchemeType::Basic(security) => {
                map.serialize_value("basic")?;
                serialize_field::<&str, S>(&security.name, "name", &mut map)?;
                serialize_field::<In, S>(&security.r#in, "in", &mut map)?;
            }
            SecuritySchemeType::Digest(security) => {
                map.serialize_value("digest")?;
                serialize_field::<&str, S>(&security.name, "name", &mut map)?;
                serialize_field::<In, S>(&security.r#in, "in", &mut map)?;
                serialize_field::<QoP, S>(&security.qop, "qop", &mut map)?;
            }
            SecuritySchemeType::Apikey(security) => {
                map.serialize_value("apikey")?;
                serialize_field::<&str, S>(&security.name, "name", &mut map)?;
                serialize_field::<In, S>(&security.r#in, "in", &mut map)?;
            }
            SecuritySchemeType::Bearer(security) => {
                map.serialize_value("bearer")?;
                serialize_field::<&str, S>(&security.authorization, "authorization", &mut map)?;
                serialize_field::<&str, S>(&security.alg, "alg", &mut map)?;
                serialize_field::<&str, S>(&security.format, "format", &mut map)?;
                serialize_field::<&str, S>(&security.name, "name", &mut map)?;
                serialize_field::<In, S>(&security.r#in, "in", &mut map)?;
            }
            SecuritySchemeType::Psk(security) => {
                map.serialize_value("psk")?;
                serialize_field::<&str, S>(&security.identity, "identity", &mut map)?;
            }
            SecuritySchemeType::Oauth2(security) => {
                map.serialize_value("oauth2")?;
                map.serialize_entry("flow", &security.flow)?;
                serialize_field::<&str, S>(&security.authorization, "authorization", &mut map)?;
                serialize_field::<&str, S>(&security.token, "token", &mut map)?;
                serialize_field::<&str, S>(&security.refresh, "refresh", &mut map)?;
                serialize_field::<ArrayEntry<&str>, S>(&security.scopes, "scopes", &mut map)?;
            }
            SecuritySchemeType::Ace(security) => {
                map.serialize_value("ace:ACESecurityScheme")?;
                serialize_field::<&str, S>(&security.authorization_server, "ace:as", &mut map)?;
                serialize_field::<&str, S>(&security.audience, "ace:audience", &mut map)?;
                serialize_field::<ArrayEntry<&str>, S>(&security.scopes, "ace:scopes", &mut map)?;
                serialize_field::<bool, S>(&security.cnonce, "ace:cnonce", &mut map)?;
            }
            SecuritySchemeType::Combo(security) => {
                map.serialize_value("combo")?;
                match &security.combo_variant {
                    ComboVariant::OneOf(one_of) => map.serialize_entry("oneOf", &one_of)?,
                    ComboVariant::AllOf(all_of) => map.serialize_entry("allOf", &all_of)?,
                }
            }
        };

        serialize_field::<ArrayEntry<&str>, S>(&self.json_ld_type, JSON_LD_TYPE, &mut map)?;
        serialize_field::<&str, S>(&self.description, "description", &mut map)?;
        serialize_field::<&'a MapEntry<&'a str>, S>(&self.descriptions, "descriptions", &mut map)?;
        serialize_field::<&str, S>(&self.proxy, "proxy", &mut map)?;

        map.end()
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum In {
    Header,
    Query,
    Body,
    Cookie,
    Auto,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum QoP {
    Auth,
    AuthInt,
}
