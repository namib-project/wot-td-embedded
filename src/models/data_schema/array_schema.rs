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

use crate::{data_structures::array::Array, serialize_field};
use serde::ser::SerializeMap;

use super::DataSchema;

#[derive(Debug)]
pub struct ArraySchema<'a> {
    pub items: Option<Array<'a, &'a DataSchema<'a>>>,
    pub min_items: Option<u64>,
    pub max_items: Option<u64>,
}

impl<'a> ArraySchema<'a> {
    pub fn serialize_to_map<S>(&self, mut map: S::SerializeMap) -> Result<S::SerializeMap, S::Error>
    where
        S: serde::Serializer,
    {
        serialize_field!("items", &self.items, map);
        serialize_field!("maxItems", &self.min_items, map);
        serialize_field!("minItems", &self.max_items, map);

        Ok(map)
    }
}
