use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::data_structures::array::Array;

#[skip_serializing_none]
#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Form<'a> {
    pub href: &'a str,
    pub content_type: Option<&'a str>,
    pub content_coding: Option<&'a str>,
    // TODO: Add operation type enum
    pub op: Option<Array<'a, &'a str>>,
    pub security: Option<Array<'a, &'a str>>,
    pub scopes: Option<Array<'a, &'a str>>,
    pub subprotocol: Option<&'a str>,
    // TODO: Add response
    // TODO: Add additionalResponses
}

#[derive(Debug, Default)]
pub struct FormBuilder<'a> {
    pub href: &'a str,
    pub content_type: Option<&'a str>,
    pub content_coding: Option<&'a str>,
    // TODO: Add operation type enum
    pub op: Option<Array<'a, &'a str>>,
    pub security: Option<Array<'a, &'a str>>,
    pub scopes: Option<Array<'a, &'a str>>,
    pub subprotocol: Option<&'a str>,
}

impl<'a> FormBuilder<'a> {
    pub fn new(href: &'a str) -> FormBuilder<'a> {
        FormBuilder {
            href,
            ..Default::default()
        }
    }

    pub fn build(self) -> Form<'a> {
        Form {
            href: self.href,
            content_type: self.content_type,
            content_coding: self.content_coding,
            op: self.op,
            security: self.security,
            scopes: self.scopes,
            subprotocol: self.subprotocol,
        }
    }
}
