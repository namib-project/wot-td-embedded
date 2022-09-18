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

pub mod action;
pub mod additional_expected_response;
pub mod data_schema;
pub mod event;
pub mod expected_response;
pub mod form;
pub mod link;
pub mod property;
pub mod security_definition;
pub mod thing_description;
pub mod version_info;

#[macro_export]
macro_rules! serialize_field {
    ($key:expr, $field:expr, $map:expr) => {
        if let Some(value) = &$field {
            $map.serialize_entry($key, value)?;
        }
    };
}
