use serde::{ser::SerializeMap, Serialize};

use crate::{
    data_structures::{array::Array, map::Map},
    serialize_field,
};

// TODO: Add builder pattern

#[derive(Debug)]
pub struct SecurityScheme<'a> {
    pub json_ld_type: Option<Array<'a, &'a str>>,
    pub description: Option<&'a str>,
    pub descriptions: Option<&'a Map<'a, &'a str>>,
    pub proxy: Option<&'a str>,
    pub scheme: SecuritySchemeType<'a>,
}

#[derive(Debug)]
pub enum SecuritySchemeType<'a> {
    Nosec,
    Combo,
    Basic(BasicSecurityScheme<'a>),
    Digest(DigestSecuritySchmeme<'a>),
    Apikey(ApiKeySecurityScheme<'a>),
    Bearer(BearerSecurityScheme<'a>),
    Psk(PskSecurityScheme<'a>),
    Oauth2(Oauth2SecurityScheme<'a>),
    Auto,
    Ace(AceSecurityScheme<'a>),
}

#[derive(Debug)]
pub struct BasicSecurityScheme<'a> {
    pub name: Option<&'a str>,
    // TODO: Replace with enum
    pub in_: Option<&'a str>,
}

#[derive(Debug)]
pub struct DigestSecuritySchmeme<'a> {
    pub name: Option<&'a str>,
    // TODO: Replace with enum
    pub in_: Option<&'a str>,
    // TODO: Replace with enum
    pub qop: Option<&'a str>,
}

#[derive(Debug)]
pub struct ApiKeySecurityScheme<'a> {
    pub name: Option<&'a str>,
    // TODO: Replace with enum
    pub in_: Option<&'a str>,
}

#[derive(Debug)]
pub struct BearerSecurityScheme<'a> {
    pub authorization: Option<&'a str>,
    pub name: Option<&'a str>,
    // TODO: Replace with enum
    pub alg: Option<&'a str>,
    pub format: Option<&'a str>,
    pub in_: Option<&'a str>,
    // TODO: Replace with enum
    pub qop: Option<&'a str>,
}

#[derive(Debug)]
pub struct PskSecurityScheme<'a> {
    pub identity: Option<&'a str>,
}

#[derive(Debug)]
pub struct Oauth2SecurityScheme<'a> {
    pub flow: &'a str,
    pub authorization: Option<&'a str>,
    pub token: Option<&'a str>,
    pub refresh: Option<&'a str>,
    pub scopes: Option<Array<'a, &'a str>>,
}

#[derive(Debug)]
pub struct AceSecurityScheme<'a> {
    pub authorization_server: Option<&'a str>,
    pub audience: Option<&'a str>,
    pub scopes: Option<Array<'a, &'a str>>,
    pub cnonce: Option<bool>,
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
                serialize_field!("in", &security.in_, map);
            }
            SecuritySchemeType::Digest(security) => {
                map.serialize_value("digest")?;
                serialize_field!("name", &security.name, map);
                serialize_field!("in", &security.in_, map);
                serialize_field!("in", &security.qop, map);
            }
            SecuritySchemeType::Apikey(security) => {
                map.serialize_value("apikey")?;
                serialize_field!("name", &security.name, map);
                serialize_field!("in", &security.in_, map);
            }
            SecuritySchemeType::Bearer(security) => {
                map.serialize_value("bearer")?;
                serialize_field!("authorization", &security.authorization, map);
                serialize_field!("name", &security.name, map);
                serialize_field!("alg", &security.alg, map);
                serialize_field!("format", &security.format, map);
                serialize_field!("in", &security.in_, map);
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
            _ => todo!(),
        };

        serialize_field!("@type", self.json_ld_type, map);
        serialize_field!("description", self.description, map);
        serialize_field!("descriptions", self.descriptions, map);
        serialize_field!("proxy", self.proxy, map);

        map.end()
    }
}
