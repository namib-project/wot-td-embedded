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

use alloc::vec::Vec;

#[derive(Debug)]
pub struct Oauth2SecurityScheme<'a> {
    pub flow: &'a str,
    pub authorization: Option<&'a str>,
    pub token: Option<&'a str>,
    pub refresh: Option<&'a str>,
    pub scopes: Option<Vec<&'a str>>,
}

impl<'a> Oauth2SecurityScheme<'a> {
    pub fn builder(flow: &'a str) -> Oauth2SecuritySchemeBuilder<'a> {
        Oauth2SecuritySchemeBuilder::new(flow)
    }
}

#[derive(Debug, Default)]
pub struct Oauth2SecuritySchemeBuilder<'a> {
    pub flow: &'a str,
    pub authorization: Option<&'a str>,
    pub token: Option<&'a str>,
    pub refresh: Option<&'a str>,
    pub scopes: Option<Vec<&'a str>>,
}

impl<'a> Oauth2SecuritySchemeBuilder<'a> {
    pub fn new(flow: &'a str) -> Self {
        Oauth2SecuritySchemeBuilder {
            flow,
            ..Default::default()
        }
    }

    pub fn authorization(mut self, authorization: &'a str) -> Self {
        self.authorization = Some(authorization);
        self
    }

    pub fn token(mut self, token: &'a str) -> Self {
        self.token = Some(token);
        self
    }

    pub fn refresh(mut self, refresh: &'a str) -> Self {
        self.refresh = Some(refresh);
        self
    }

    pub fn scopes(mut self, scopes: Vec<&'a str>) -> Self {
        self.scopes = Some(scopes);
        self
    }

    pub fn build(self) -> Oauth2SecurityScheme<'a> {
        Oauth2SecurityScheme {
            flow: self.flow,
            authorization: self.authorization,
            token: self.token,
            refresh: self.refresh,
            scopes: self.scopes,
        }
    }
}
