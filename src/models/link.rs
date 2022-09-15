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

use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::data_structures::array::Array;

#[skip_serializing_none]
#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Link<'a> {
    pub href: &'a str,
    #[serde(rename = "type")]
    pub link_type: Option<&'a str>,
    pub rel: Option<&'a str>,
    pub anchor: Option<&'a str>,
    pub sizes: Option<&'a str>,
    pub hreflang: Option<Array<'a, &'a str>>,
}

impl<'a> Link<'a> {
    pub fn builder() -> LinkBuilder<'a> {
        LinkBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct LinkBuilder<'a> {
    pub href: &'a str,
    pub link_type: Option<&'a str>,
    pub rel: Option<&'a str>,
    pub anchor: Option<&'a str>,
    pub sizes: Option<&'a str>,
    pub hreflang: Option<Array<'a, &'a str>>,
}

impl<'a> LinkBuilder<'a> {
    pub fn new(href: &'a str) -> LinkBuilder<'a> {
        LinkBuilder {
            href,
            ..Default::default()
        }
    }

    pub fn href(mut self, href: &'a str) -> LinkBuilder<'a> {
        self.href = href;
        self
    }

    pub fn link_type(mut self, link_type: &'a str) -> LinkBuilder<'a> {
        self.link_type = Some(link_type);
        self
    }

    pub fn rel(mut self, rel: &'a str) -> LinkBuilder<'a> {
        self.rel = Some(rel);
        self
    }

    pub fn anchor(mut self, anchor: &'a str) -> LinkBuilder<'a> {
        self.anchor = Some(anchor);
        self
    }

    pub fn hreflang(mut self, hreflang: Array<'a, &'a str>) -> LinkBuilder<'a> {
        self.hreflang = Some(hreflang);
        self
    }

    pub fn build(self) -> Link<'a> {
        Link {
            href: self.href,
            link_type: self.link_type,
            rel: self.rel,
            anchor: self.anchor,
            sizes: self.sizes,
            hreflang: self.hreflang,
        }
    }
}
