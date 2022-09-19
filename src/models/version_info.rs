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
use serde_with::skip_serializing_none;

use crate::serialization::{JsonString, JsonValue, SerializableField, SerializationError};

#[skip_serializing_none]
#[derive(Serialize, Debug, Default)]
pub struct VersionInfo<'a> {
    pub instance: &'a str,
    pub model: Option<&'a str>,
}

impl<'a> VersionInfo<'a> {
    pub fn new(instance: &'a str) -> VersionInfo<'a> {
        VersionInfo {
            instance,
            ..Default::default()
        }
    }

    pub fn builder(instance: &'a str) -> VersionInfoBuilder<'a> {
        VersionInfoBuilder::new(instance)
    }
}

#[derive(Debug, Default)]
pub struct VersionInfoBuilder<'a> {
    pub instance: &'a str,
    pub model: Option<&'a str>,
}

impl<'a> VersionInfoBuilder<'a> {
    pub fn new(instance: &'a str) -> VersionInfoBuilder<'a> {
        VersionInfoBuilder {
            instance,
            ..Default::default()
        }
    }

    pub fn instance(mut self, instance: &'a str) -> VersionInfoBuilder<'a> {
        self.instance = instance;
        self
    }

    pub fn model(mut self, model: &'a str) -> VersionInfoBuilder<'a> {
        self.model = Some(model);
        self
    }

    pub fn build(self) -> VersionInfo<'a> {
        VersionInfo {
            instance: self.instance,
            model: self.model,
        }
    }
}

impl<'a> JsonValue for VersionInfo<'a> {
    fn to_json_value(&self, buf: &mut [u8], index: usize) -> Result<usize, SerializationError> {
        let mut index = "{".to_json_string(buf, index)?;

        index = self
            .instance
            .serialize_field("instance", buf, index, false)?;

        index = self.model.serialize_field("model", buf, index, true)?;

        index = "}".to_json_string(buf, index)?;

        Ok(index)
    }
}

#[cfg(test)]
mod tests {
    use serde_json_core::{heapless::String, ser::Error, to_string};

    use super::VersionInfo;

    #[test]
    fn serialize() -> Result<(), Error> {
        let additional_expected_response = VersionInfo::builder("v1.0.0").model("v0.1.0").build();

        let expected_result = r#"{"instance":"v1.0.0","model":"v0.1.0"}"#;
        let actual_result: String<50> = to_string(&additional_expected_response)?;

        assert_eq!(expected_result, actual_result.as_str());
        Ok(())
    }
}
