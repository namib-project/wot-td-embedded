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

// pub mod array;
// pub mod map;

use core::slice::Iter;

use alloc::vec::Vec;
use serde::{ser::SerializeMap, Serialize};

pub type Array<T> = Vec<T>;
// pub type Map<'a, T> = Vec<(&'a str, T)>;
#[derive(Debug, Default)]
pub struct Map<'a, T: Serialize> {
    entries: Vec<(&'a str, T)>,
}

impl<'a, T: Serialize> Map<'a, T> {
    pub fn new(entries: Vec<(&'a str, T)>) -> Self {
        Map { entries }
    }

    pub fn iter(&'a self) -> Iter<'a, (&'a str, T)> {
        self.entries.iter()
    }

    pub fn find_value(&mut self, key: &'a str) -> Option<&T> {
        for item in self.entries.iter() {
            if item.0 == key {
                return Some(&item.1);
            }
        }

        None
    }

    pub fn find_index(&mut self, key: &'a str) -> Option<usize> {
        let mut index: usize = 0;

        for item in self.entries.iter() {
            if item.0 == key {
                return Some(index);
            }
            index = index + 1;
        }

        None
    }

    pub fn insert(&mut self, key: &'a str, value: T) -> Option<T> {
        let mut removed: Option<T> = None;
        let index = self.find_index(key);

        if let Some(index) = index {
            removed = Some(self.entries.remove(index).1);
        }

        self.entries.push((key, value));

        removed
    }
}

impl<'a, T: Serialize> Serialize for Map<'a, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut sequence = serializer.serialize_map(None)?;

        for item in self.entries.iter() {
            sequence.serialize_entry(item.0, &item.1)?;
        }

        sequence.end()
    }
}
