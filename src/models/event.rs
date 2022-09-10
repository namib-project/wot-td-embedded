use serde::Serialize;
use serde_with::skip_serializing_none;

use super::data_schema::DataSchema;

#[skip_serializing_none]
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Event<'a> {
    pub subscription: Option<DataSchema<'a>>,
    pub data: Option<DataSchema<'a>>,
    #[serde(rename = "dataResponse")]
    pub data_response: Option<DataSchema<'a>>,
    pub cancellation: Option<DataSchema<'a>>,
}
