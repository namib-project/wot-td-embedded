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

pub type Array<'a, T> = ArrayEntry<'a, T>;

#[derive(Debug)]
pub struct ArrayEntry<'a, T: Serialize>
where
    T: Serialize,
{
    value: T,
    index: usize,
    next: Option<&'a mut ArrayEntry<'a, T>>,
}

impl<'a, T: Serialize> ArrayEntry<'a, T> {
    pub fn new(value: T) -> ArrayEntry<'a, T> {
        ArrayEntry {
            value,
            index: 0,
            next: None,
        }
    }

    fn get_last_entry(&mut self) -> &mut ArrayEntry<'a, T> {
        let mut current_entry = self;

        loop {
            if current_entry.next.is_none() {
                break;
            }

            current_entry = current_entry
                .next
                .as_mut()
                .expect("Expected a next element");
        }

        current_entry
    }

    pub fn set_next(&mut self, next: &'a mut ArrayEntry<'a, T>) -> &ArrayEntry<'a, T> {
        self.next = Some(next);
        self
    }

    pub fn add_entry(mut self, entry: &'a mut ArrayEntry<'a, T>) -> ArrayEntry<'a, T> {
        let last_entry = self.get_last_entry();
        entry.index = last_entry.index + 1;
        self.get_last_entry().set_next(entry);
        self
    }
}

impl<'a, T: Serialize> Serialize for ArrayEntry<'a, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut sequence = serializer.serialize_seq(None)?;

        let mut current_entry = self;

        loop {
            sequence.serialize_element(&current_entry.value)?;

            if current_entry.next.is_none() {
                break;
            }

            current_entry = current_entry
                .next
                .as_ref()
                .expect("Expected a next element");
        }

        sequence.end()
    }
}
