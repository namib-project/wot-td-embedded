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

use alloc::vec::Vec;

use crate::models::serialize_field;

use super::DataSchema;

#[derive(Debug)]
pub struct ArraySchema<'a> {
    pub items: Option<Vec<&'a DataSchema<'a>>>,
    pub min_items: Option<u64>,
    pub max_items: Option<u64>,
}

impl<'a> ArraySchema<'a> {
    pub fn serialize_to_map<S>(&self, mut map: S::SerializeMap) -> Result<S::SerializeMap, S::Error>
    where
        S: serde::Serializer,
    {
        serialize_field::<Vec<&'a DataSchema>, S>(&self.items, "items", &mut map)?;
        serialize_field::<u64, S>(&self.min_items, "maxItems", &mut map)?;
        serialize_field::<u64, S>(&self.max_items, "minItems", &mut map)?;

        Ok(map)
    }
}
