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
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Link<'a> {
    pub href: &'a str,
    #[serde(rename = "@type")]
    pub link_type: Option<&'a str>,
    pub rel: Option<&'a str>,
    pub anchor: Option<&'a str>,
    pub sizes: Option<&'a str>,
    pub hreflang: Option<Vec<&'a str>>,
}

impl<'a> Link<'a> {
    pub fn builder(href: &'a str) -> LinkBuilder<'a> {
        LinkBuilder::new(href)
    }

    pub fn new(href: &'a str) -> Self {
        Link {
            href,
            ..Default::default()
        }
    }
}

#[derive(Debug, Default)]
pub struct LinkBuilder<'a> {
    pub href: &'a str,
    pub link_type: Option<&'a str>,
    pub rel: Option<&'a str>,
    pub anchor: Option<&'a str>,
    pub sizes: Option<&'a str>,
    pub hreflang: Option<Vec<&'a str>>,
}

impl<'a> LinkBuilder<'a> {
    pub fn new(href: &'a str) -> Self {
        LinkBuilder {
            href,
            ..Default::default()
        }
    }

    pub fn href(mut self, href: &'a str) -> Self {
        self.href = href;
        self
    }

    pub fn link_type(mut self, link_type: &'a str) -> Self {
        self.link_type = Some(link_type);
        self
    }

    pub fn rel(mut self, rel: &'a str) -> Self {
        self.rel = Some(rel);
        self
    }

    pub fn anchor(mut self, anchor: &'a str) -> Self {
        self.anchor = Some(anchor);
        self
    }

    pub fn hreflang(mut self, hreflang: Vec<&'a str>) -> Self {
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

#[cfg(test)]
mod tests {
    use alloc::vec;
    use serde_json_core::{heapless::String, ser::Error, to_string};

    use super::Link;

    #[test]
    fn serialize() -> Result<(), Error> {
        let additional_expected_response = Link::builder("coap://example.org")
            .link_type("test:testLink")
            .rel("test")
            .anchor("test")
            .hreflang(vec!["de"])
            .build();

        let expected_result = r#"{"href":"coap://example.org","@type":"test:testLink","rel":"test","anchor":"test","hreflang":["de"]}"#;
        let actual_result: String<100> = to_string(&additional_expected_response)?;

        assert_eq!(expected_result, actual_result.as_str());
        Ok(())
    }
}
