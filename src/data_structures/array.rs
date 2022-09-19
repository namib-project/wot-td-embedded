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

use serde::{ser::SerializeSeq, Serialize};

use crate::serialization::{JsonString, JsonValue};

pub type Array<'a, T> = ArrayEntry<'a, T>;

#[derive(Debug, Default)]
pub struct ArrayEntry<'a, T> {
    value: T,
    next: Option<&'a ArrayEntry<'a, T>>,
}

impl<'a, T> ArrayEntry<'a, T> {
    pub fn new(value: T) -> Self {
        ArrayEntry { value, next: None }
    }

    pub fn add(value: T, next: &'a ArrayEntry<'a, T>) -> Self {
        ArrayEntry {
            value,
            next: Some(next),
        }
    }

    pub fn iter(&'a self) -> Iter<'a, T> {
        Iter {
            current: Some(self),
        }
    }
}

pub struct Iter<'a, T> {
    current: Option<&'a ArrayEntry<'a, T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        match self.current {
            Some(ArrayEntry { value, next }) => {
                self.current = *next;
                Some(value)
            }
            None => None,
        }
    }
}

impl<'a, T: Serialize> Serialize for ArrayEntry<'a, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut sequence = serializer.serialize_seq(None)?;

        for item in self.iter() {
            sequence.serialize_element(item)?;
        }

        sequence.end()
    }
}

impl<'a, T: JsonValue> JsonValue for ArrayEntry<'a, T> {
    fn to_json_value(
        &self,
        buf: &mut [u8],
        index: usize,
    ) -> Result<usize, crate::serialization::SerializationError> {
        let mut has_previous = false;
        let mut index = index;

        index = "[".to_json_string(buf, index)?;

        for entry in self.iter() {
            if has_previous {
                index = ",".to_json_string(buf, index)?;
            }
            index = entry.to_json_value(buf, index)?;
            has_previous = true;
        }

        index = "]".to_json_string(buf, index)?;

        Ok(index)
    }
}
