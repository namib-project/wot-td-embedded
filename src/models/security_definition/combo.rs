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

use crate::data_structures::Array;

#[derive(Debug)]
pub struct ComboSecurityScheme<'a> {
    pub combo_variant: ComboVariant<'a>,
}

impl<'a> ComboSecurityScheme<'a> {
    pub fn new(combo_variant: ComboVariant<'a>) -> ComboSecurityScheme<'a> {
        ComboSecurityScheme { combo_variant }
    }
}

#[derive(Debug)]
pub enum ComboVariant<'a> {
    AllOf(Array<&'a str>),
    OneOf(Array<&'a str>),
}
