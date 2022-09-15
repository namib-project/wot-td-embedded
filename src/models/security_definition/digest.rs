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

#[derive(Debug, Default)]
pub struct DigestSecurityScheme<'a> {
    pub name: Option<&'a str>,
    pub r#in: Option<In>,
    pub qop: Option<QoP>,
}

impl<'a> DigestSecurityScheme<'a> {
    pub fn builder() -> DigestSecuritySchemeBuilder<'a> {
        DigestSecuritySchemeBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct DigestSecuritySchemeBuilder<'a> {
    pub name: Option<&'a str>,
    pub r#in: Option<In>,
    pub qop: Option<QoP>,
}

impl<'a> DigestSecuritySchemeBuilder<'a> {
    pub fn new() -> Self {
        DigestSecuritySchemeBuilder::default()
    }

    pub fn name(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }

    pub fn in_(mut self, r#in: In) -> Self {
        self.r#in = Some(r#in);
        self
    }

    pub fn qop(mut self, qop: QoP) -> Self {
        self.qop = Some(qop);
        self
    }

    pub fn build(self) -> DigestSecurityScheme<'a> {
        DigestSecurityScheme {
            name: self.name,
            r#in: self.r#in,
            qop: self.qop,
        }
    }
}
