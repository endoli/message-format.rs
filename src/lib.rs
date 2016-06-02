// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Message Format
//!
//! A [`Message`] is a piece of user-visible text that typically has
//! variable elements.
//!
//! This crate provides [ICU-style message formatting]. This provides
//! for formatting values taking localization rules into account. ICU
//! Message Format is widely supported in many languages and environments.
//! This library will endeavor to support all of the ICU Message Format
//! with the exception of the deprecated `ChoiceFormat`.
//!
//! [`Message`]: struct.Message.html
//! [ICU-style message formatting]: http://userguide.icu-project.org/formatparse/messages

#![warn(missing_docs)]
#![deny(trivial_numeric_casts,
        unsafe_code, unstable_features,
        unused_import_braces, unused_qualifications)]

#[macro_use]
extern crate nom;

pub mod icu;
mod args;
mod format;
mod message;
mod value;

pub use self::args::{arg, Args};
pub use self::format::Format;
pub use self::message::Message;
pub use self::value::{AsValue, Value};
