use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Debug, Default)]
pub struct VersionInfo<'a> {
    pub instance: &'a str,
    pub model: Option<&'a str>,
}
