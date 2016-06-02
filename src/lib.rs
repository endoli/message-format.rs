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
//! This module currently provides support for messages using the
//! [ICU Message Format]. In the future, we hope to support other
//! message formats as well, especially L20n.
//!
//! [ICU Message Format]: icu/index.html
//! [`Message`]: struct.Message.html

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
