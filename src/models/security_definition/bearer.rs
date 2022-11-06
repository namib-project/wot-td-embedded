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

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{In, QoP};

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BearerSecurityScheme<'a> {
    pub authorization: Option<&'a str>,
    pub name: Option<&'a str>,
    pub alg: Option<&'a str>,
    pub format: Option<&'a str>,
    pub r#in: Option<In>,
    pub qop: Option<QoP>,
}

impl<'a> BearerSecurityScheme<'a> {
    pub fn builder() -> BearerSecuritySchemeBuilder<'a> {
        BearerSecuritySchemeBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct BearerSecuritySchemeBuilder<'a> {
    pub authorization: Option<&'a str>,
    pub name: Option<&'a str>,
    pub alg: Option<&'a str>,
    pub format: Option<&'a str>,
    pub r#in: Option<In>,
    pub qop: Option<QoP>,
}

impl<'a> BearerSecuritySchemeBuilder<'a> {
    pub fn new() -> Self {
        BearerSecuritySchemeBuilder::default()
    }

    pub fn authorization(mut self, authorization: &'a str) -> Self {
        self.authorization = Some(authorization);
        self
    }

    pub fn name(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }

    pub fn alg(mut self, alg: &'a str) -> Self {
        self.alg = Some(alg);
        self
    }

    pub fn format(mut self, format: &'a str) -> Self {
        self.format = Some(format);
        self
    }

    pub fn r#in(mut self, r#in: In) -> Self {
        self.r#in = Some(r#in);
        self
    }

    pub fn qop(mut self, qop: QoP) -> Self {
        self.qop = Some(qop);
        self
    }

    pub fn build(self) -> BearerSecurityScheme<'a> {
        BearerSecurityScheme {
            authorization: self.authorization,
            name: self.name,
            alg: self.alg,
            format: self.format,
            r#in: self.r#in,
            qop: self.qop,
        }
    }
}
