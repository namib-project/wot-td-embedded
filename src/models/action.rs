use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::data_structures::array::Array;

use super::{data_schema::DataSchema, form::Form};

#[skip_serializing_none]
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Action<'a> {
    pub forms: Array<'a, Form<'a>>,
    pub input: Option<&'a DataSchema<'a>>,
    pub output: Option<&'a DataSchema<'a>>,
    pub safe: Option<bool>,
    pub idempotent: Option<bool>,
    pub synchronous: Option<bool>,
}

impl<'a> Action<'a> {
    pub fn builder() -> ActionBuilder<'a> {
        todo!()
    }
}

#[derive(Debug)]

pub struct ActionBuilder<'a> {
    pub forms: Array<'a, Form<'a>>,
    pub input: Option<&'a DataSchema<'a>>,
    pub output: Option<&'a DataSchema<'a>>,
    pub safe: Option<bool>,
    pub idempotent: Option<bool>,
    pub synchronous: Option<bool>,
}

impl<'a> ActionBuilder<'a> {
    pub fn new(forms: Array<'a, Form<'a>>) -> ActionBuilder<'a> {
        ActionBuilder {
            forms,
            input: None,
            output: None,
            safe: None,
            idempotent: None,
            synchronous: None,
        }
    }

    pub fn input(mut self, input: &'a DataSchema<'a>) -> ActionBuilder<'a> {
        self.input = Some(input);
        self
    }

    pub fn output(mut self, output: &'a DataSchema<'a>) -> ActionBuilder<'a> {
        self.output = Some(output);
        self
    }

    pub fn safe(mut self, safe: bool) -> ActionBuilder<'a> {
        self.safe = Some(safe);
        self
    }

    pub fn idempotent(mut self, idempotent: bool) -> ActionBuilder<'a> {
        self.idempotent = Some(idempotent);
        self
    }

    pub fn synchronous(mut self, synchronous: bool) -> ActionBuilder<'a> {
        self.synchronous = Some(synchronous);
        self
    }

    pub fn build(self) -> Action<'a> {
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
