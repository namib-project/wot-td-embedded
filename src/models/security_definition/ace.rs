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
pub struct AceSecurityScheme<'a> {
    pub authorization_server: Option<&'a str>,
    pub audience: Option<&'a str>,
    pub scopes: Option<Array<&'a str>>,
    pub cnonce: Option<bool>,
}

impl<'a> AceSecurityScheme<'a> {
    pub fn builder() -> AceSecuritySchemeBuilder<'a> {
        AceSecuritySchemeBuilder::new()
    }
}

#[derive(Debug, Default)]
pub struct AceSecuritySchemeBuilder<'a> {
    pub authorization_server: Option<&'a str>,
    pub audience: Option<&'a str>,
    pub scopes: Option<Array<&'a str>>,
    pub cnonce: Option<bool>,
}

impl<'a> AceSecuritySchemeBuilder<'a> {
    pub fn new() -> Self {
        AceSecuritySchemeBuilder::default()
    }

    pub fn authorization_server(mut self, authorization_server: &'a str) -> Self {
        self.authorization_server = Some(authorization_server);
        self
    }

    pub fn audience(mut self, audience: &'a str) -> Self {
        self.audience = Some(audience);
        self
    }
    pub fn scopes(mut self, scopes: Array<&'a str>) -> Self {
        self.scopes = Some(scopes);
        self
    }

    pub fn cnonce(mut self, cnonce: bool) -> Self {
        self.cnonce = Some(cnonce);
        self
    }

    pub fn build(self) -> AceSecurityScheme<'a> {
        AceSecurityScheme {
            authorization_server: self.authorization_server,
            audience: self.audience,
            scopes: self.scopes,
            cnonce: self.cnonce,
        }
    }
}
