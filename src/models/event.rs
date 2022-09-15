use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::data_structures::{array::Array, map::Map};

use super::{data_schema::DataSchema, form::Form};

#[skip_serializing_none]
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Event<'a> {
    pub forms: Array<'a, Form<'a>>,
    pub json_ld_type: Option<Array<'a, &'a str>>,
    pub title: Option<&'a str>,
    pub titles: Option<&'a Map<'a, &'a str>>,
    pub description: Option<&'a str>,
    pub descriptions: Option<&'a Map<'a, &'a str>>,
    pub subscription: Option<&'a DataSchema<'a>>,
    pub data: Option<&'a DataSchema<'a>>,
    pub data_response: Option<&'a DataSchema<'a>>,
    pub cancellation: Option<&'a DataSchema<'a>>,
}

impl<'a> Event<'a> {
    pub fn builder() -> EventBuilder<'a> {
        todo!()
    }
}

#[derive(Debug)]
pub struct EventBuilder<'a> {
    pub forms: Array<'a, Form<'a>>,
    pub json_ld_type: Option<Array<'a, &'a str>>,
    pub title: Option<&'a str>,
    pub titles: Option<&'a Map<'a, &'a str>>,
    pub description: Option<&'a str>,
    pub descriptions: Option<&'a Map<'a, &'a str>>,
    pub subscription: Option<&'a DataSchema<'a>>,
    pub data: Option<&'a DataSchema<'a>>,
    pub data_response: Option<&'a DataSchema<'a>>,
    pub cancellation: Option<&'a DataSchema<'a>>,
}

impl<'a> EventBuilder<'a> {
    pub fn new(forms: Array<'a, Form<'a>>) -> EventBuilder<'a> {
        EventBuilder {
            forms,
            json_ld_type: None,
            title: None,
            titles: None,
            description: None,
            descriptions: None,
            subscription: None,
            data: None,
            data_response: None,
            cancellation: None,
        }
    }

    pub fn subscription(mut self, subscription: &'a DataSchema<'a>) -> EventBuilder<'a> {
        self.subscription = Some(subscription);
        self
    }

    pub fn data(mut self, data: &'a DataSchema<'a>) -> EventBuilder<'a> {
        self.data = Some(data);
        self
    }

    pub fn data_response(mut self, data_response: &'a DataSchema<'a>) -> EventBuilder<'a> {
        self.data_response = Some(data_response);
        self
    }

    pub fn cancellation(mut self, cancellation: &'a DataSchema<'a>) -> EventBuilder<'a> {
        self.cancellation = Some(cancellation);
        self
    }

    pub fn build(self) -> Event<'a> {
        Event {
            forms: self.forms,
            json_ld_type: self.json_ld_type,
            title: self.title,
            titles: self.titles,
            description: self.description,
            descriptions: self.descriptions,
            subscription: self.subscription,
            data: self.data,
            data_response: self.data_response,
            cancellation: self.cancellation,
        }
    }
}
