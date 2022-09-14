use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::data_structures::array::Array;

use super::{data_schema::DataSchema, form::Form};

#[skip_serializing_none]
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Event<'a> {
    pub forms: Array<'a, Form<'a>>,
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
    pub subscription: Option<&'a DataSchema<'a>>,
    pub data: Option<&'a DataSchema<'a>>,
    pub data_response: Option<&'a DataSchema<'a>>,
    pub cancellation: Option<&'a DataSchema<'a>>,
}

impl<'a> EventBuilder<'a> {
    pub fn new(forms: Array<'a, Form<'a>>) -> EventBuilder<'a> {
        EventBuilder {
            forms,
            subscription: None,
            data: None,
            data_response: None,
            cancellation: None,
        }
    }

    pub fn subscription(&mut self, subscription: &'a DataSchema<'a>) -> &EventBuilder<'a> {
        self.subscription = Some(subscription);
        self
    }

    pub fn data(&mut self, data: &'a DataSchema<'a>) -> &EventBuilder<'a> {
        self.data = Some(data);
        self
    }

    pub fn data_response(&mut self, data_response: &'a DataSchema<'a>) -> &EventBuilder<'a> {
        self.data_response = Some(data_response);
        self
    }

    pub fn cancellation(&mut self, cancellation: &'a DataSchema<'a>) -> &EventBuilder<'a> {
        self.cancellation = Some(cancellation);
        self
    }

    pub fn build(self) -> Event<'a> {
        Event {
            forms: self.forms,
            subscription: self.subscription,
            data: self.data,
            data_response: self.data_response,
            cancellation: self.cancellation,
        }
    }
}
