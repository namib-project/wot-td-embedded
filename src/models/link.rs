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

use crate::{
    data_structures::array::Array,
    serialization::{JsonString, JsonValue, SerializableField, SerializationError},
};

#[skip_serializing_none]
#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Link<'a> {
    pub href: &'a str,
    #[serde(rename = "@type")]
    pub link_type: Option<&'a str>,
    pub rel: Option<&'a str>,
    pub anchor: Option<&'a str>,
    pub sizes: Option<&'a str>,
    pub hreflang: Option<Array<'a, &'a str>>,
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

    pub fn sizes(mut self, sizes: &'a str) -> LinkBuilder<'a> {
        self.sizes = Some(sizes);
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

impl<'a> JsonValue for Link<'a> {
    fn to_json_value(&self, buf: &mut [u8], index: usize) -> Result<usize, SerializationError> {
        let mut new_index = "{".to_json_string(buf, index)?;

        new_index = self.href.serialize_field("href", buf, new_index, false)?;

        new_index = self
            .link_type
            .serialize_field("@type", buf, new_index, true)?;

        new_index = self.rel.serialize_field("rel", buf, new_index, true)?;

        new_index = self
            .anchor
            .serialize_field("anchor", buf, new_index, true)?;

        new_index = self.sizes.serialize_field("sizes", buf, new_index, true)?;

        new_index = self
            .hreflang
            .serialize_field("hreflang", buf, new_index, true)?;

        new_index = "}".to_json_string(buf, new_index)?;

        Ok(new_index)
    }
}

#[cfg(test)]
mod tests {
    use serde_json_core::{heapless::String, ser::Error, to_string};

    use crate::{
        data_structures::array::Array,
        serialization::{JsonValue, SerializationError},
    };

    use super::Link;

    #[test]
    fn serialize() -> Result<(), Error> {
        let hreflang = Array::<&str>::new("de");

        let link = Link::builder("coap://example.org")
            .link_type("test:testLink")
            .rel("test")
            .anchor("test")
            .hreflang(hreflang)
            .build();

        let expected_result = r#"{"href":"coap://example.org","@type":"test:testLink","rel":"test","anchor":"test","hreflang":["de"]}"#;
        let actual_result: String<100> = to_string(&link)?;

        assert_eq!(expected_result, actual_result.as_str());
        Ok(())
    }

    #[test]
    fn serialize_to_buffer() -> Result<(), SerializationError> {
        let hreflang = Array::<&str>::new("de");

        let link = Link::builder("coap://example.org")
            .link_type("test:testLink")
            .rel("test")
            .anchor("test")
            .hreflang(hreflang)
            .build();

        let mut buffer: [u8; 200] = [0; 200];

        let length = link.to_json_value(&mut buffer, 0)?;

        assert_eq!(r#"{"href":"coap://example.org","@type":"test:testLink","rel":"test","anchor":"test","hreflang":["de"]}"#.as_bytes(), &buffer[0..length]);

        Ok(())
    }
}
