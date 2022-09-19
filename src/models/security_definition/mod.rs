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
    data_structures::{array::Array, map::Map},
    serialization::{JsonString, JsonValue, SerializableField, SerializationError},
    serialize_field,
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

impl<'a> JsonValue for SecurityScheme<'a> {
    fn to_json_value(
        &self,
        buf: &mut [u8],
        index: usize,
    ) -> Result<usize, crate::serialization::SerializationError> {
        let mut index = "{".to_json_string(buf, index)?;

        index = self.scheme.serialize_field("scheme", buf, index, false)?;

        index = "}".to_json_string(buf, index)?;

        Ok(index)
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
                serialize_field!("name", &security.name, map);
                serialize_field!("in", &security.r#in, map);
            }
            SecuritySchemeType::Digest(security) => {
                map.serialize_value("digest")?;
                serialize_field!("name", &security.name, map);
                serialize_field!("in", &security.r#in, map);
                serialize_field!("qop", &security.qop, map);
            }
            SecuritySchemeType::Apikey(security) => {
                map.serialize_value("apikey")?;
                serialize_field!("name", &security.name, map);
                serialize_field!("in", &security.r#in, map);
            }
            SecuritySchemeType::Bearer(security) => {
                map.serialize_value("bearer")?;
                serialize_field!("authorization", &security.authorization, map);
                serialize_field!("name", &security.name, map);
                serialize_field!("alg", &security.alg, map);
                serialize_field!("format", &security.format, map);
                serialize_field!("in", &security.r#in, map);
            }
            SecuritySchemeType::Psk(security) => {
                map.serialize_value("psk")?;
                serialize_field!("identity", &security.identity, map);
            }
            SecuritySchemeType::Oauth2(security) => {
                map.serialize_value("oauth2")?;
                map.serialize_entry("flow", &security.flow)?;
                serialize_field!("authorization", &security.authorization, map);
                serialize_field!("token", &security.token, map);
                serialize_field!("refresh", &security.refresh, map);
                serialize_field!("scopes", &security.scopes, map);
            }
            SecuritySchemeType::Ace(security) => {
                map.serialize_value("ace:ACESecurityScheme")?;
                serialize_field!("ace:as", &security.authorization_server, map);
                serialize_field!("ace:audience", &security.audience, map);
                serialize_field!("ace:scopes", &security.scopes, map);
                serialize_field!("ace:cnonce", &security.cnonce, map);
            }
            SecuritySchemeType::Combo(security) => {
                map.serialize_value("combo")?;
                match &security.combo_variant {
                    ComboVariant::OneOf(one_of) => map.serialize_entry("oneOf", &one_of)?,
                    ComboVariant::AllOf(all_of) => map.serialize_entry("allOf", &all_of)?,
                }
            }
        };

        serialize_field!(JSON_LD_TYPE, self.json_ld_type, map);
        serialize_field!("description", self.description, map);
        serialize_field!("descriptions", self.descriptions, map);
        serialize_field!("proxy", self.proxy, map);

        map.end()
    }
}

impl<'a> JsonValue for SecuritySchemeType<'a> {
    fn to_json_value(&self, buf: &mut [u8], index: usize) -> Result<usize, SerializationError> {
        match self {
            SecuritySchemeType::Nosec => {
                "nosec".to_json_value(buf, index)?;
            }
            SecuritySchemeType::Auto => {
                "auto".to_json_value(buf, index)?;
            }
            SecuritySchemeType::Combo(security) => {
                "combo".to_json_value(buf, index)?;
            }
            SecuritySchemeType::Basic(security) => {
                "basic".to_json_value(buf, index)?;
                security.name.serialize_field("name", buf, index, true)?;
                security.r#in.serialize_field("in", buf, index, true)?;
            }
            SecuritySchemeType::Digest(security) => {
                "digest".to_json_value(buf, index)?;
                security.name.serialize_field("name", buf, index, true)?;
                security.r#in.serialize_field("in", buf, index, true)?;
                security.qop.serialize_field("qop", buf, index, true)?;
            }
            SecuritySchemeType::Apikey(security) => {
                "apikey".to_json_value(buf, index)?;
                security.name.serialize_field("name", buf, index, true)?;
                security.r#in.serialize_field("in", buf, index, true)?;
            }
            SecuritySchemeType::Bearer(security) => {
                "bearer".to_json_value(buf, index)?;
                security
                    .authorization
                    .serialize_field("name", buf, index, true)?;
                security.name.serialize_field("name", buf, index, true)?;
                security.alg.serialize_field("alg", buf, index, true)?;
                security
                    .format
                    .serialize_field("format", buf, index, true)?;
                security.r#in.serialize_field("in", buf, index, true)?;
            }
            SecuritySchemeType::Psk(security) => {
                "psk".to_json_value(buf, index)?;
                security
                    .identity
                    .serialize_field("identity", buf, index, true)?;
            }
            SecuritySchemeType::Oauth2(security) => {
                "oauth2".to_json_value(buf, index)?;
                security.flow.serialize_field("flow", buf, index, true)?;
                security
                    .authorization
                    .serialize_field("authorization", buf, index, true)?;
                security.token.serialize_field("token", buf, index, true)?;
                security
                    .refresh
                    .serialize_field("refresh", buf, index, true)?;
                security
                    .scopes
                    .serialize_field("scopes", buf, index, true)?;
            }
            SecuritySchemeType::Ace(security) => {
                "ace:ACESecurityScheme".to_json_value(buf, index)?;
                security
                    .authorization_server
                    .serialize_field("ace:as", buf, index, true)?;
                security
                    .audience
                    .serialize_field("ace:audience", buf, index, true)?;
                security
                    .scopes
                    .serialize_field("ace:scopes", buf, index, true)?;
                security
                    .cnonce
                    .serialize_field("ace:cnonce", buf, index, true)?;
            }
        }

        Ok(index)
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

impl JsonValue for In {
    fn to_json_value(&self, buf: &mut [u8], index: usize) -> Result<usize, SerializationError> {
        match self {
            In::Header => "header".to_json_value(buf, index),
            In::Query => "query".to_json_value(buf, index),
            In::Body => "body".to_json_value(buf, index),
            In::Cookie => "cookie".to_json_value(buf, index),
            In::Auto => "auto".to_json_value(buf, index),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum QoP {
    Auth,
    AuthInt,
}

impl JsonValue for QoP {
    fn to_json_value(&self, buf: &mut [u8], index: usize) -> Result<usize, SerializationError> {
        match self {
            QoP::Auth => "auth".to_json_value(buf, index),
            QoP::AuthInt => "auth-int".to_json_value(buf, index),
        }
    }
}
