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

use super::In;
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeySecurityScheme<'a> {
    pub name: Option<&'a str>,
    pub r#in: Option<In>,
}

impl<'a> ApiKeySecurityScheme<'a> {
    pub fn builder() -> ApiKeySecuritySchemeBuilder<'a> {
        ApiKeySecuritySchemeBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct ApiKeySecuritySchemeBuilder<'a> {
    pub name: Option<&'a str>,
    pub r#in: Option<In>,
}

impl<'a> ApiKeySecuritySchemeBuilder<'a> {
    pub fn new() -> Self {
        ApiKeySecuritySchemeBuilder::default()
    }

    pub fn name(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }

    pub fn r#in(mut self, r#in: In) -> Self {
        self.r#in = Some(r#in);
        self
    }

    pub fn build(self) -> ApiKeySecurityScheme<'a> {
        ApiKeySecurityScheme {
            name: self.name,
            r#in: self.r#in,
        }
    }
}
