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

//! Serializable WoT Thing Description data structures for embedded devices
//! and other `no_std` environments.
//!
//! This crate is an implementation of the Web of Things Thing Description
//! (WoT TD) 1.1, specified by the W3C. Using `no_std` data types, it is
//! especially geared towards the use with embedded devices that do not allow
//! for heap allocation. As all defined structs implement Serde's [`Serialize`]
//! trait, you can easily serialize a Thing Description as JSON using the
//! `serde_json_core` crate.
//!
//! If you have plan on using WoT TD in an `std` environment, we recommend using
//! the `wot-td` crate instead as it contains additional validation checks and
//! is easier to use than this crate.
//!
//! # Example
//!
//! A simple example for the creation of a Thing Description struct and its
//! serialization can be seen below.
//!
//! ```
//! assert!(2 + 2 == 4, "Replace me with a real example!");
//! ```
//!
//! # Usage
//! ```toml
//! [dependencies]
//! wot_td_embedded = { version = "^0.1" }
//! ```
//!

#![no_std]

pub mod constants;
pub mod data_structures;
pub mod models;
