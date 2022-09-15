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

#[derive(Debug)]
pub struct Oauth2SecurityScheme<'a> {
    pub flow: &'a str,
    pub authorization: Option<&'a str>,
    pub token: Option<&'a str>,
    pub refresh: Option<&'a str>,
    pub scopes: Option<Array<'a, &'a str>>,
}
