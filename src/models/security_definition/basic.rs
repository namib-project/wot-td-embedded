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

use super::In;

#[derive(Debug)]
pub struct BasicSecurityScheme<'a> {
    pub name: Option<&'a str>,
    pub r#in: Option<In>,
}

impl<'a> BasicSecurityScheme<'a> {
    pub fn builder() -> BasicSecuritySchemeBuilder<'a> {
        BasicSecuritySchemeBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct BasicSecuritySchemeBuilder<'a> {
    pub name: Option<&'a str>,
    pub r#in: Option<In>,
}

impl<'a> BasicSecuritySchemeBuilder<'a> {
    pub fn new() -> Self {
        BasicSecuritySchemeBuilder::default()
    }

    pub fn name(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }

    pub fn r#in(mut self, r#in: In) -> Self {
        self.r#in = Some(r#in);
        self
    }

    pub fn build(self) -> BasicSecurityScheme<'a> {
        BasicSecurityScheme {
            name: self.name,
            r#in: self.r#in,
        }
    }
}
