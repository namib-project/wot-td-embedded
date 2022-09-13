use heapless::Vec;
use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Form<'a, const OP: usize = 2, const SECURITY: usize = 0, const SCOPES: usize = 0> {
    pub href: &'a str,
    pub content_type: Option<&'a str>,
    pub content_coding: Option<&'a str>,
    pub op: Option<&'a Vec<&'a str, OP>>,
    pub security: Option<&'a Vec<&'a str, SECURITY>>,
    pub scopes: Option<&'a Vec<&'a str, SCOPES>>,
    pub subprotocol: Option<&'a str>,
    // TODO: Add response
    // TODO: Add additionalResponses
}

#[derive(Debug, Default)]
pub struct FormBuilder<'a, const OP: usize = 0, const SECURITY: usize = 0, const SCOPES: usize = 0>
{
    pub href: &'a str,
    pub content_type: Option<&'a str>,
    pub content_coding: Option<&'a str>,
    pub op: Option<&'a Vec<&'a str, OP>>,
    pub security: Option<&'a Vec<&'a str, SECURITY>>,
    pub scopes: Option<&'a Vec<&'a str, SCOPES>>,
    pub subprotocol: Option<&'a str>,
}

impl<'a, const OP: usize, const SECURITY: usize, const SCOPES: usize>
    FormBuilder<'a, OP, SECURITY, SCOPES>
{
    pub fn new(href: &'a str) -> FormBuilder<'a, OP, SECURITY, SCOPES> {
        FormBuilder {
            href,
            ..Default::default()
        }
    }

    pub fn build(&self) -> Form<'a, OP, SECURITY, SCOPES> {
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
