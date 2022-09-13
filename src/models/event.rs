use heapless::Vec;
use serde::Serialize;
use serde_with::skip_serializing_none;

use super::{data_schema::DataSchema, form::Form};

#[skip_serializing_none]
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Event<'a, const FORMS: usize = 2> {
    pub forms: &'a Vec<Form<'a>, FORMS>,
    pub subscription: Option<&'a DataSchema<'a>>,
    pub data: Option<&'a DataSchema<'a>>,
    pub data_response: Option<&'a DataSchema<'a>>,
    pub cancellation: Option<&'a DataSchema<'a>>,
}

impl<'a, const FORMS: usize> Event<'a, FORMS> {
    pub fn builder() -> EventBuilder<'a, FORMS> {
        todo!()
    }
}

#[derive(Debug)]
pub struct EventBuilder<'a, const FORMS: usize = 2> {
    pub forms: &'a Vec<Form<'a>, FORMS>,
    pub subscription: Option<&'a DataSchema<'a>>,
    pub data: Option<&'a DataSchema<'a>>,
    pub data_response: Option<&'a DataSchema<'a>>,
    pub cancellation: Option<&'a DataSchema<'a>>,
}

impl<'a, const FORMS: usize> EventBuilder<'a, FORMS> {
    pub fn new(forms: &'a Vec<Form, FORMS>) -> EventBuilder<'a, FORMS> {
        EventBuilder {
            forms,
            subscription: None,
            data: None,
            data_response: None,
            cancellation: None,
        }
    }

    pub fn subscription(&mut self, subscription: &'a DataSchema<'a>) -> &EventBuilder<'a, FORMS> {
        self.subscription = Some(subscription);
        self
    }

    pub fn data(&mut self, data: &'a DataSchema<'a>) -> &EventBuilder<'a, FORMS> {
        self.data = Some(data);
        self
    }

    pub fn data_response(&mut self, data_response: &'a DataSchema<'a>) -> &EventBuilder<'a, FORMS> {
        self.data_response = Some(data_response);
        self
    }

    pub fn cancellation(&mut self, cancellation: &'a DataSchema<'a>) -> &EventBuilder<'a, FORMS> {
        self.cancellation = Some(cancellation);
        self
    }

    pub fn build(&self) -> Event<'a, FORMS> {
        Event {
            forms: self.forms,
            subscription: self.subscription,
            data: self.data,
            data_response: self.data_response,
            cancellation: self.cancellation,
        }
    }
}
