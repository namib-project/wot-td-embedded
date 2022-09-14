use heapless::Vec;
use serde::{ser::SerializeMap, Serialize};

use super::{data_schema::DataSchema, form::Form};

#[derive(Debug)]
pub struct Property<'a, const FORMS: usize = 2> {
    pub forms: &'a Vec<Form<'a>, FORMS>,
    pub data_schema: &'a DataSchema<'a>,
    pub observable: Option<bool>,
}

impl<'a, const FORMS: usize> Property<'a, FORMS> {
    pub fn builder() -> PropertyBuilder<'a, FORMS> {
        todo!()
    }
}

impl<'a> Serialize for Property<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(None)?;

        map.serialize_entry("forms", self.forms)?;

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
pub struct PropertyBuilder<'a, const FORMS: usize = 2> {
    pub forms: &'a Vec<Form<'a>, FORMS>,
    pub data_schema: &'a DataSchema<'a>,
    pub observable: Option<bool>,
}

impl<'a, const FORMS: usize> PropertyBuilder<'a, FORMS> {
    pub fn new(
        forms: &'a Vec<Form, FORMS>,
        data_schema: &'a DataSchema<'a>,
    ) -> PropertyBuilder<'a, FORMS> {
        PropertyBuilder {
            forms,
            data_schema,
            observable: None,
        }
    }

    pub fn observable(&mut self, observable: bool) -> &PropertyBuilder<'a, FORMS> {
        self.observable = Some(observable);
        self
    }

    pub fn build(self) -> Property<'a, FORMS> {
        Property {
            forms: self.forms,
            data_schema: self.data_schema,
            observable: self.observable,
        }
    }
}
