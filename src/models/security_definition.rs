use serde::{ser::SerializeMap, Serialize};
use serde_with::skip_serializing_none;

// TODO: Add more fields and security scheme types

#[derive(Debug)]
pub struct SecurityScheme<'a> {
    pub description: Option<&'a str>,
    pub scheme: SecuritySchemeType,
}

#[skip_serializing_none]
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum SecuritySchemeType {
    Nosec,
}

#[derive(Debug, Default)]
pub struct NoSecurityScheme {}

impl<'a> Serialize for SecurityScheme<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(None)?;

        map.serialize_key("scheme")?;

        match self.scheme {
            SecuritySchemeType::Nosec => {
                map.serialize_value("nosec")?;
            }
        };

        if self.description.is_some() {
            map.serialize_entry("description", self.description.unwrap())?;
        }

        map.end()
    }
}
