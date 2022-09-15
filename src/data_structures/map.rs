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

use serde::{ser::SerializeMap, Serialize};

#[derive(Debug, Default)]
pub struct Map<'a, T: Serialize> {
    root: Option<&'a MapEntry<'a, T>>,
}

impl<'a, T: Serialize> Serialize for Map<'a, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if self.root.is_none() {
            return serializer.serialize_map(Some(0))?.end();
        }

        self.root.serialize(serializer)
    }
}

impl<'a, T: Serialize> Map<'a, T> {
    pub fn new() -> Map<'a, T> {
        Map { root: None }
    }

    pub fn insert(mut self, entry: &'a mut MapEntry<'a, T>) -> Map<'a, T> {
        entry.next = self.root;
        self.root = Some(entry);

        self
    }
}

#[derive(Debug, Default)]
pub struct MapEntry<'a, T>
where
    T: Serialize,
{
    key: &'a str,
    value: T,
    next: Option<&'a MapEntry<'a, T>>,
}

impl<'a, T: Serialize> MapEntry<'a, T> {
    pub fn new(key: &'a str, value: T) -> MapEntry<T> {
        MapEntry {
            key,
            value,
            next: None,
        }
    }
}

impl<'a, T: Serialize> Serialize for MapEntry<'a, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(None)?;

        let mut current_entry = self;

        loop {
            map.serialize_entry(current_entry.key, &current_entry.value)?;

            if current_entry.next.is_none() {
                break;
            }

            current_entry = current_entry
                .next
                .as_ref()
                .expect("Expected a next element");
        }

        map.end()
    }
}
