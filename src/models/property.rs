use serde::{ser::SerializeMap, Serialize};

use super::data_schema::DataSchema;

#[derive(Debug)]
pub struct Property<'a> {
    // TODO: Deal with nested serialization
    pub data_schema: DataSchema<'a>,
    pub observable: Option<bool>,
}

impl<'a> Serialize for Property<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(None)?;

        if self.observable.is_some() {
            map.serialize_entry(
                "observable",
                &self.observable.expect("Expected observable to be Some!"),
            )?;
        }

        // TODO: Add complete dataschema serialization

        let data_schema = &self.data_schema;

        if data_schema.title.is_some() {
            map.serialize_entry("title", data_schema.title.unwrap())?;
        }

        map.end()
    }
}
