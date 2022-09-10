use serde::Serialize;
use serde_with::skip_serializing_none;

use super::data_schema::DataSchema;

#[skip_serializing_none]
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Action<'a> {
    pub input: Option<DataSchema<'a>>,
    pub output: Option<DataSchema<'a>>,
    pub safe: Option<bool>,
    pub idempotent: Option<bool>,
    pub synchronous: Option<bool>,
}
