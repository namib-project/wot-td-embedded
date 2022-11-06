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
pub struct ExpectedResponse {
    pub content_type: String,
}

impl ExpectedResponse {
    pub fn new(content_type: String) -> Self {
        ExpectedResponse { content_type }
    }
}
#[cfg(test)]
mod tests {
    use alloc::borrow::ToOwned;
    use serde_json_core::{heapless::String, ser::Error, to_string};

    use super::ExpectedResponse;

    #[test]
    fn serialize() -> Result<(), Error> {
        let additional_expected_response = ExpectedResponse::new("application/json".to_owned());

        let expected_result = r#"{"contentType":"application/json"}"#;
        let actual_result: String<34> = to_string(&additional_expected_response)?;

        assert_eq!(expected_result, actual_result.as_str());
        Ok(())
    }
}
