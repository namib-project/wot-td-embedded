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

// pub type Map<'a, T> = MapEntry<'a, T>;

#[derive(Debug, Default)]
pub struct Map<'a, T> {
    first: Option<&'a MapEntry<'a, T>>,
}

impl<'a, T> Map<'a, T> {
    pub fn new() -> Self {
        Map { first: None }
    }

    pub fn add(mut self, next: &'a mut MapEntry<'a, T>) -> Self {
        match self.first {
            Some(first) => {
                next.next = Some(first);
                self.first = Some(next);
            }
            None => self.first = Some(next),
        }

        self
    }

    pub fn new_entry(key: &'a str, value: T) -> MapEntry<'a, T> {
        MapEntry {
            key,
            value,
            next: None,
        }
    }

    pub fn iter(&'a self) -> Iter<'a, T> {
        Iter {
            current: self.first,
        }
    }
}

#[derive(Debug, Default)]
pub struct MapEntry<'a, T> {
    key: &'a str,
    value: T,
    next: Option<&'a MapEntry<'a, T>>,
}

impl<'a, T> MapEntry<'a, T> {
    pub fn new(key: &'a str, value: T) -> Self {
        MapEntry {
            key,
            value,
            next: None,
        }
    }

    pub fn add(key: &'a str, value: T, next: &'a MapEntry<'a, T>) -> Self {
        MapEntry {
            key,
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
    current: Option<&'a MapEntry<'a, T>>,
}

pub struct MapItem<'a, T> {
    pub key: &'a str,
    pub value: &'a T,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = MapItem<'a, T>;
    fn next(&mut self) -> Option<MapItem<'a, T>> {
        match self.current {
            Some(MapEntry { key, value, next }) => {
                self.current = *next;
                Some(MapItem { key, value })
            }
            None => None,
        }
    }
}

impl<'a, T: Serialize> Serialize for Map<'a, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if let Some(first) = self.first {
            return first.serialize(serializer);
        }

        let sequence = serializer.serialize_map(None)?;
        sequence.end()
    }
}

impl<'a, T: Serialize> Serialize for MapEntry<'a, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut sequence = serializer.serialize_map(None)?;

        for item in self.iter() {
            sequence.serialize_entry(item.key, item.value)?;
        }

        sequence.end()
    }
}
