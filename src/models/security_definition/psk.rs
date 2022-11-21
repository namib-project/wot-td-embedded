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

#[derive(Debug)]
pub struct PskSecurityScheme<'a> {
    pub identity: Option<&'a str>,
}

impl<'a> PskSecurityScheme<'a> {
    pub fn builder() -> PskSecuritySchemeBuilder<'a> {
        PskSecuritySchemeBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct PskSecuritySchemeBuilder<'a> {
    pub identity: Option<&'a str>,
}

impl<'a> PskSecuritySchemeBuilder<'a> {
    pub fn new() -> Self {
        PskSecuritySchemeBuilder::default()
    }

    pub fn authorization(mut self, identity: &'a str) -> Self {
        self.identity = Some(identity);
        self
    }

    pub fn build(self) -> PskSecurityScheme<'a> {
        PskSecurityScheme {
            identity: self.identity,
        }
    }
}
