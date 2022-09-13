use serde::{ser::SerializeMap, Serialize};

use super::data_schema::DataSchema;

#[derive(Debug)]
pub struct Property<'a> {
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

        self.data_schema.serialize_to_map::<S>(map)?.end()
    }
}
