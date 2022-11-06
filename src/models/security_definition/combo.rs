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

use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComboSecurityScheme {
    pub combo_variant: ComboVariant,
}

impl ComboSecurityScheme {
    pub fn new(combo_variant: ComboVariant) -> ComboSecurityScheme {
        ComboSecurityScheme { combo_variant }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComboVariant {
    AllOf(Vec<String>),
    OneOf(Vec<String>),
}
