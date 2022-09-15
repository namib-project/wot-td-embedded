use serde::Serialize;

#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExpectedResponse<'a> {
    pub content_type: &'a str,
}

impl<'a> ExpectedResponse<'a> {
    pub fn new(content_type: &'a str) -> Self {
        ExpectedResponse { content_type }
    }
}
