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

use crate::serialize_field;
use serde::ser::SerializeMap;

#[derive(Debug)]
pub struct NumberSchema {
    pub minimum: Option<f64>,
    pub maximum: Option<f64>,
    pub exclusive_minimum: Option<f64>,
    pub exclusive_maximum: Option<f64>,
    pub multiple_of: Option<f64>,
}

impl NumberSchema {
    pub fn serialize_to_map<S>(&self, mut map: S::SerializeMap) -> Result<S::SerializeMap, S::Error>
    where
        S: serde::Serializer,
    {
        serialize_field!("minimum", &self.minimum, map);
        serialize_field!("maximum", &self.maximum, map);
        serialize_field!("exclusiveMinimum", &self.exclusive_minimum, map);
        serialize_field!("exclusiveMaximum", &self.exclusive_maximum, map);
        serialize_field!("multipleOf", &self.multiple_of, map);

        Ok(map)
    }
}
