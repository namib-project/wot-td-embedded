/*
 * Copyright (c) 2022 The NAMIB Project Developers.
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 *
 * SPDX-License-Identifier: MIT OR Apache-2.0
 */

use serde::Serialize;

use crate::serialization::{JsonString, JsonValue, SerializableField};

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

impl<'a> JsonValue for AdditionalExpectedResponse<'a> {
    fn to_json_value(
        &self,
        buf: &mut [u8],
        index: usize,
    ) -> Result<usize, crate::serialization::SerializationError> {
        let mut index = "{".to_json_string(buf, index)?;

        index = self.success.serialize_field("success", buf, index, false)?;
        let mut has_previous = self.success.is_some();

        index = self
            .content_type
            .serialize_field("contentType", buf, index, has_previous)?;
        has_previous |= self.content_type.is_some();

        index = self
            .schema
            .serialize_field("schema", buf, index, has_previous)?;

        index = "}".to_json_string(buf, index)?;

        Ok(index)
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

#[cfg(test)]
mod tests {

    use serde_json_core::{heapless::String, ser::Error, to_string};

    use super::AdditionalExpectedResponse;

    #[test]
    fn serialize() -> Result<(), Error> {
        let additional_expected_response = AdditionalExpectedResponse::builder()
            .content_type("application/json")
            .success(true)
            .schema("test")
            .build();

        let expected_result =
            r#"{"success":true,"contentType":"application/json","schema":"test"}"#;
        let actual_result: String<65> = to_string(&additional_expected_response)?;

        assert_eq!(expected_result, actual_result.as_str());
        Ok(())
    }
}
