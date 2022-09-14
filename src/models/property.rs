use serde::{ser::SerializeMap, Serialize};

use crate::data_structures::array::Array;

use super::{data_schema::DataSchema, form::Form};

#[derive(Debug)]
pub struct Property<'a> {
    pub forms: Array<'a, Form<'a>>,
    pub data_schema: DataSchema<'a>,
    pub observable: Option<bool>,
}

impl<'a> Property<'a> {
    pub fn builder() -> PropertyBuilder<'a> {
        todo!()
    }
}

impl<'a> Serialize for Property<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(None)?;

        map.serialize_entry("forms", &self.forms)?;

        if self.observable.is_some() {
            map.serialize_entry(
                "observable",
                &self.observable.expect("Expected observable to be Some!"),
            )?;
        }

        self.data_schema.serialize_to_map::<S>(map)?.end()
    }
}

#[derive(Debug)]
pub struct PropertyBuilder<'a> {
    pub forms: Array<'a, Form<'a>>,
    pub data_schema: DataSchema<'a>,
    pub observable: Option<bool>,
}

impl<'a> PropertyBuilder<'a> {
    pub fn new(forms: Array<'a, Form<'a>>, data_schema: DataSchema<'a>) -> PropertyBuilder<'a> {
        PropertyBuilder {
            forms,
            data_schema,
            observable: None,
        }
    }

    pub fn observable(&mut self, observable: bool) -> &PropertyBuilder<'a> {
        self.observable = Some(observable);
        self
    }

    pub fn data_schema(&mut self, data_schema: DataSchema<'a>) -> &PropertyBuilder<'a> {
        self.data_schema = data_schema;
        self
    }

    pub fn build(self) -> Property<'a> {
        Property {
            forms: self.forms,
            data_schema: self.data_schema,
            observable: self.observable,
        }
    }
}
