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
        serialize_field::<f64, S>(&self.minimum, "minimum", &mut map)?;
        serialize_field::<f64, S>(&self.maximum, "maximum", &mut map)?;
        serialize_field::<f64, S>(&self.exclusive_minimum, "exclusiveMinimum", &mut map)?;
        serialize_field::<f64, S>(&self.exclusive_maximum, "exclusiveMaximum", &mut map)?;
        serialize_field::<f64, S>(&self.multiple_of, "multipleOf", &mut map)?;

        Ok(map)
    }
}
