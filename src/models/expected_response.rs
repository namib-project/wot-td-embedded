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
pub struct ExpectedResponse<'a> {
    pub content_type: &'a str,
}

impl<'a> ExpectedResponse<'a> {
    pub fn new(content_type: &'a str) -> Self {
        ExpectedResponse { content_type }
    }
}

impl<'a> JsonValue for ExpectedResponse<'a> {
    fn to_json_value(
        &self,
        buf: &mut [u8],
        index: usize,
    ) -> Result<usize, crate::serialization::SerializationError> {
        let mut index = "{".to_json_string(buf, index)?;

        index = self
            .content_type
            .serialize_field("contentType", buf, index, false)?;

        index = "}".to_json_string(buf, index)?;

        Ok(index)
    }
}

#[cfg(test)]
mod tests {
    use serde_json_core::{heapless::String, ser::Error, to_string};

    use super::ExpectedResponse;

    #[test]
    fn serialize() -> Result<(), Error> {
        let additional_expected_response = ExpectedResponse::new("application/json");

        let expected_result = r#"{"contentType":"application/json"}"#;
        let actual_result: String<34> = to_string(&additional_expected_response)?;

        assert_eq!(expected_result, actual_result.as_str());
        Ok(())
    }
}
