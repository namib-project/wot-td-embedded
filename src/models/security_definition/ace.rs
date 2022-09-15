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

use crate::data_structures::array::Array;

// TODO: Add builder pattern

#[derive(Debug)]
pub struct AceSecurityScheme<'a> {
    pub authorization_server: Option<&'a str>,
    pub audience: Option<&'a str>,
    pub scopes: Option<Array<'a, &'a str>>,
    pub cnonce: Option<bool>,
}
