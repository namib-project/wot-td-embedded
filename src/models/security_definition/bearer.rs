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

use super::{In, QoP};

#[derive(Debug)]
pub struct BearerSecurityScheme<'a> {
    pub authorization: Option<&'a str>,
    pub name: Option<&'a str>,
    pub alg: Option<&'a str>,
    pub format: Option<&'a str>,
    pub in_: Option<In>,
    pub qop: Option<QoP>,
}
