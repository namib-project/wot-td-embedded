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

use alloc::string::String;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::In;

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicSecurityScheme {
    pub name: Option<String>,
    pub r#in: Option<In>,
}

impl BasicSecurityScheme {
    pub fn builder() -> BasicSecuritySchemeBuilder {
        BasicSecuritySchemeBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct BasicSecuritySchemeBuilder {
    pub name: Option<String>,
    pub r#in: Option<In>,
}

impl BasicSecuritySchemeBuilder {
    pub fn new() -> Self {
        BasicSecuritySchemeBuilder::default()
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn r#in(mut self, r#in: In) -> Self {
        self.r#in = Some(r#in);
        self
    }

    pub fn build(self) -> BasicSecurityScheme {
        BasicSecurityScheme {
            name: self.name,
            r#in: self.r#in,
        }
    }
}
