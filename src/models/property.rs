use serde::Serialize;
use serde_with::skip_serializing_none;

use super::data_schema::DataSchema;

#[skip_serializing_none]
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Property<'a> {
    // TODO: Deal with nested serialization
    data_schema: DataSchema<'a>,
    observable: Option<bool>,
}
