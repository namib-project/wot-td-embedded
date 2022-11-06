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

use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalExpectedResponse {
    pub success: Option<bool>,
    pub content_type: Option<String>,
    pub schema: Option<String>,
}

impl AdditionalExpectedResponse {
    pub fn builder() -> AdditionalExpectedResponseBuilder {
        AdditionalExpectedResponseBuilder::new()
    }
}

#[derive(Debug, Default)]
pub struct AdditionalExpectedResponseBuilder {
    pub success: Option<bool>,
    pub content_type: Option<String>,
    pub schema: Option<String>,
}

impl AdditionalExpectedResponseBuilder {
    pub fn new() -> Self {
        AdditionalExpectedResponseBuilder::default()
    }

    pub fn success(mut self, success: bool) -> Self {
        self.success = Some(success);
        self
    }

    pub fn content_type(mut self, content_type: String) -> Self {
        self.content_type = Some(content_type);
        self
    }

    pub fn schema(mut self, schema: String) -> Self {
        self.schema = Some(schema);
        self
    }

    pub fn build(self) -> AdditionalExpectedResponse {
        AdditionalExpectedResponse {
            success: self.success,
            content_type: self.content_type,
            schema: self.schema,
        }
    }
}

#[cfg(test)]
mod tests {

    use alloc::borrow::ToOwned;
    use serde_json_core::{heapless::String, ser::Error, to_string};

    use super::AdditionalExpectedResponse;

    #[test]
    fn serialize() -> Result<(), Error> {
        let additional_expected_response = AdditionalExpectedResponse::builder()
            .content_type("application/json".to_owned())
            .success(true)
            .schema("test".to_owned())
            .build();

        let expected_result =
            r#"{"success":true,"contentType":"application/json","schema":"test"}"#;
        let actual_result: String<65> = to_string(&additional_expected_response)?;

        assert_eq!(expected_result, actual_result.as_str());
        Ok(())
    }
}
