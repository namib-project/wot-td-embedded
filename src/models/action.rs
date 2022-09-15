use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::data_structures::{array::Array, map::Map};

use super::{data_schema::DataSchema, form::Form};

#[skip_serializing_none]
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Action<'a> {
    pub forms: Array<'a, Form<'a>>,
    #[serde(rename = "@type")]
    pub json_ld_type: Option<Array<'a, &'a str>>,
    pub title: Option<&'a str>,
    pub titles: Option<&'a Map<'a, &'a str>>,
    pub description: Option<&'a str>,
    pub descriptions: Option<&'a Map<'a, &'a str>>,
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
    pub json_ld_type: Option<Array<'a, &'a str>>,
    pub title: Option<&'a str>,
    pub titles: Option<&'a Map<'a, &'a str>>,
    pub description: Option<&'a str>,
    pub descriptions: Option<&'a Map<'a, &'a str>>,
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
            json_ld_type: None,
            title: None,
            titles: None,
            description: None,
            descriptions: None,
            input: None,
            output: None,
            safe: None,
            idempotent: None,
            synchronous: None,
        }
    }

    pub fn json_ld_type(mut self, json_ld_type: Array<'a, &'a str>) -> ActionBuilder<'a> {
        self.json_ld_type = Some(json_ld_type);
        self
    }

    pub fn title(mut self, title: &'a str) -> ActionBuilder<'a> {
        self.title = Some(title);
        self
    }

    pub fn titles(mut self, titles: &'a Map<'a, &'a str>) -> ActionBuilder<'a> {
        self.titles = Some(titles);
        self
    }

    pub fn description(mut self, description: &'a str) -> ActionBuilder<'a> {
        self.description = Some(description);
        self
    }

    pub fn descriptions(mut self, descriptions: &'a Map<'a, &'a str>) -> ActionBuilder<'a> {
        self.descriptions = Some(descriptions);
        self
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
            json_ld_type: self.json_ld_type,
            title: self.title,
            titles: self.titles,
            description: self.description,
            descriptions: self.descriptions,
            input: self.input,
            output: self.output,
            safe: self.safe,
            idempotent: self.idempotent,
            synchronous: self.synchronous,
        }
    }
}
