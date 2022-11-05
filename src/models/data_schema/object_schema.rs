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

use super::DataSchema;
use crate::data_structures::array::ArrayEntry;
use crate::data_structures::map::MapEntry;
use crate::data_structures::{array::Array, map::Map};
use crate::models::serialize_field;

#[derive(Debug)]
pub struct ObjectSchema<'a> {
    pub properties: Option<Map<'a, &'a DataSchema<'a>>>,
    pub required: Option<Array<'a, &'a str>>,
}

impl<'a> ObjectSchema<'a> {
    pub fn serialize_to_map<S>(&self, mut map: S::SerializeMap) -> Result<S::SerializeMap, S::Error>
    where
        S: serde::Serializer,
    {
        serialize_field::<Map<&'a DataSchema>, S>(&self.properties, "properties", &mut map)?;
        serialize_field::<Array<&'a str>, S>(&self.required, "required", &mut map)?;

        Ok(map)
    }
}
