use heapless::Vec;
use serde::Serialize;
use serde_with::skip_serializing_none;

use super::{data_schema::DataSchema, form::Form};

#[skip_serializing_none]
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Action<'a, const FORMS: usize = 2> {
    pub forms: &'a Vec<Form<'a>, FORMS>,
    pub input: Option<&'a DataSchema<'a>>,
    pub output: Option<&'a DataSchema<'a>>,
    pub safe: Option<bool>,
    pub idempotent: Option<bool>,
    pub synchronous: Option<bool>,
}

impl<'a, const FORMS: usize> Action<'a, FORMS> {
    pub fn builder() -> ActionBuilder<'a, FORMS> {
        todo!()
    }
}

#[derive(Debug)]

pub struct ActionBuilder<'a, const FORMS: usize = 2> {
    pub forms: &'a Vec<Form<'a>, FORMS>,
    pub input: Option<&'a DataSchema<'a>>,
    pub output: Option<&'a DataSchema<'a>>,
    pub safe: Option<bool>,
    pub idempotent: Option<bool>,
    pub synchronous: Option<bool>,
}

impl<'a, const FORMS: usize> ActionBuilder<'a, FORMS> {
    pub fn new(forms: &'a Vec<Form, FORMS>) -> ActionBuilder<'a, FORMS> {
        ActionBuilder {
            forms,
            input: None,
            output: None,
            safe: None,
            idempotent: None,
            synchronous: None,
        }
    }

    pub fn input(&mut self, input: &'a DataSchema<'a>) -> &ActionBuilder<'a, FORMS> {
        self.input = Some(input);
        self
    }

    pub fn output(&mut self, output: &'a DataSchema<'a>) -> &ActionBuilder<'a, FORMS> {
        self.output = Some(output);
        self
    }

    pub fn safe(&mut self, safe: bool) -> &ActionBuilder<'a, FORMS> {
        self.safe = Some(safe);
        self
    }

    pub fn idempotent(&mut self, idempotent: bool) -> &ActionBuilder<'a, FORMS> {
        self.idempotent = Some(idempotent);
        self
    }

    pub fn synchronous(&mut self, synchronous: bool) -> &ActionBuilder<'a, FORMS> {
        self.synchronous = Some(synchronous);
        self
    }

    pub fn build(&self) -> Action<'a, FORMS> {
        Action {
            forms: self.forms,
            input: self.input,
            output: self.output,
            safe: self.safe,
            idempotent: self.idempotent,
            synchronous: self.synchronous,
        }
    }
}
