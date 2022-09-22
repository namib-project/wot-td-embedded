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

use crate::models::serialize_field;

#[derive(Debug)]
pub struct StringSchema<'a> {
    pub min_length: Option<u64>,
    pub max_length: Option<u64>,
    pub pattern: Option<&'a str>,
    pub content_encoding: Option<&'a str>,
    pub content_media_type: Option<&'a str>,
}

impl<'a> StringSchema<'a> {
    pub fn serialize_to_map<S>(&self, mut map: S::SerializeMap) -> Result<S::SerializeMap, S::Error>
    where
        S: serde::Serializer,
    {
        serialize_field::<u64, S>(&self.min_length, "minLength", &mut map)?;
        serialize_field::<u64, S>(&self.max_length, "maxLength", &mut map)?;
        serialize_field::<&str, S>(&self.pattern, "pattern", &mut map)?;
        serialize_field::<&str, S>(&self.content_encoding, "contentEncoding", &mut map)?;
        serialize_field::<&str, S>(&self.content_media_type, "contentMediaType", &mut map)?;

        Ok(map)
    }
}
