use serde::Serialize;

#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalExpectedResponse<'a> {
    pub success: Option<bool>,
    pub content_type: Option<&'a str>,
    pub schema: Option<&'a str>,
}

impl<'a> AdditionalExpectedResponse<'a> {
    pub fn builder() -> AdditionalExpectedResponseBuilder<'a> {
        AdditionalExpectedResponseBuilder::new()
    }
}

#[derive(Debug, Default)]
pub struct AdditionalExpectedResponseBuilder<'a> {
    pub success: Option<bool>,
    pub content_type: Option<&'a str>,
    pub schema: Option<&'a str>,
}

impl<'a> AdditionalExpectedResponseBuilder<'a> {
    pub fn new() -> Self {
        AdditionalExpectedResponseBuilder::default()
    }

    pub fn success(mut self, success: bool) -> Self {
        self.success = Some(success);
        self
    }

    pub fn content_type(mut self, content_type: &'a str) -> Self {
        self.content_type = Some(content_type);
        self
    }

    pub fn schema(mut self, schema: &'a str) -> Self {
        self.schema = Some(schema);
        self
    }

    pub fn build(self) -> AdditionalExpectedResponse<'a> {
        AdditionalExpectedResponse {
            success: self.success,
            content_type: self.content_type,
            schema: self.schema,
        }
    }
}
