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
//! Often, when writing your application, you need to customize the
//! output based on various variable values:
//!
//! > Your search had no results.
//!
//! > Your search had one result.
//!
//! > Your search had 3 results.
//!
//! You don't want to have specific code for each message output to
//! build up the text, and ideally, you'll be supporting localization
//! so that your application can be used by people who speak other
//! languages, which often have different rules concerning pluralization,
//! gender and list formatting.
//!
//! Separating messages from the code has other benefits as well apart
//! from making localization easier. You may need different branding of
//! your product for different builds, or you may want to be able to have
//! someone edit and proofread the text without having to have them modify
//! the code itself.
//!
//! This module currently provides support for messages using the
//! [ICU Message Format]. In the future, we hope to support other
//! message formats as well, especially L20n.
//!
//! ## Installation
//!
//! This crate works with Cargo and is on
//! [crates.io](https://crates.io/crates/message-format).
//! Add it to your `Cargo.toml` like so:
//!
//! ```toml
//! [dependencies]
//! message-format = "0.0.1"
//! ```
//!
//! ## Messages
//!
//! The simplest way to create a [`Message`] from code is to [`parse`] it
//! from a text format:
//!
//! ```
//! use message_format::*;
//!
//! let m = icu::parse("Connecting to {host}...").unwrap();
//! ```
//!
//! For details on the [ICU Message Format] syntax, see the [`icu` module].
//!
//! ## Arguments
//!
//! Messages need arguments or parameters. Since messages typically have named
//! arguments, we can't just pass arguments directly like we might do with
//! `format!` or other lower level formatting operations. Instead, we construct
//! a set of [`Args`]:
//!
//! ```
//! use message_format::*;
//!
//! let m = icu::parse("Connecting to {host}...").unwrap();
//! assert_eq!(&m.format_message(&arg("host", "localhost")),
//!            "Connecting to localhost...");
//! ```
//!
//! Multiple arguments can be provided by chaining together calls to `arg`:
//!
//! ```
//! use message_format::*;
//!
//! let m = icu::parse("{name} went to {place}.").unwrap();
//! assert_eq!(&m.format_message(&arg("name", "Jacob").arg("place", "the store")),
//!            "Jacob went to the store.");
//! ```
//! ## Future Directions
//!
//! In the future, we want to extend this library to support a number of
//! additional features:
//!
//! * Serializing and deserializing messages in a binary format to avoid
//!   needing to parse them on every application startup.
//! * Integration with `rust-locale` or other libraries for doing locale
//!   specific formatting.
//! * Extending the types of data that can be used with [`Value`].
//! * Supporting [L20n] and perhaps other message format syntaxes. (This
//!   will probably require API changes to support the concept of a
//!   context among other things.)
//! * Offline utilities for compiling and validating message format
//!   strings, converting to and from various formats like XLIFF, etc.
//!
//! ## Contributions
//!
//! Contributions are welcome.
//!
//! [`Args`]: struct.Args.html
//! [ICU Message Format]: icu/index.html
//! [`icu` module]: icu/index.html
//! [L20n]: http://l20n.org/
//! [`parse`]: icu/fn.parse.html
//! [`Message`]: struct.Message.html
//! [`Value`]: enum.Value.html

#![warn(missing_docs)]
#![deny(trivial_numeric_casts,
        unsafe_code, unstable_features,
        unused_import_braces, unused_qualifications)]

#[macro_use]
extern crate nom;

pub mod icu;
mod args;
mod context;
mod message;
mod message_part;
mod plural_category;
mod plural_classifiers;
mod value;

pub use self::args::{arg, Args};
pub use self::context::Context;
pub use self::message::Message;
pub use self::message_part::MessagePart;
pub use self::plural_category::PluralCategory;
pub use self::plural_classifiers::*;
pub use self::value::{AsValue, Value};
