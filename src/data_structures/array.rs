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

#[derive(Debug, Default)]
pub struct Array<T> {
    first: Option<&'a ArrayEntry<'a, T>>,
    last: Option<&'a ArrayEntry<'a, T>>,
}

impl<'a, T> Array<T> {
    pub fn new() -> Self {
        Array {
            first: None,
            last: None,
        }
    }

    pub fn add(mut self, next: &'a mut ArrayEntry<'a, T>) -> Self {
        match self.first {
            Some(first) => {
                first.next = Some(&mut next);
                next.next = Some(first);
                self.first = Some(next);
            }
            None => self.first = Some(next),
        }

        self
    }

    pub fn new_entry(value: T) -> ArrayEntry<'a, T> {
        ArrayEntry { value, next: None }
    }

    pub fn iter(&'a self) -> Iter<'a, T> {
        Iter {
            current: self.first,
        }
    }
}

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

impl<'a, T: Serialize> Serialize for Array<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if let Some(first) = self.first {
            return first.serialize(serializer);
        }

        let sequence = serializer.serialize_seq(None)?;
        sequence.end()
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
