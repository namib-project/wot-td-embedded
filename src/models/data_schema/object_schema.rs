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
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

use super::DataSchema;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectSchema<'a> {
    #[serde(borrow)]
    pub properties: Option<HashMap<&'a str, DataSchema<'a>>>,
    pub required: Option<Vec<&'a str>>,
}
