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

use std::fmt;

pub mod ast;
mod args;
mod parse_message;

use ast::Format;
pub use self::args::{arg, Args};
pub use self::parse_message::parse_message;

/// A message that has been localized and can be formatted in a
/// locale-aware manner.
pub struct Message {
    parts: Vec<Box<Format>>,
}

impl Message {
    /// Construct a message from constituent parts.
    pub fn new(parts: Vec<Box<Format>>) -> Self {
        Message { parts: parts }
    }

    /// Format a message to a stream.
    pub fn format_message<'f>(&'f self,
                              stream: &mut fmt::Write,
                              args: &'f Args<'f>)
                              -> fmt::Result {
        for part in &self.parts {
            try!(part.apply_format(stream, args));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let m = Message::new(vec![Box::new(ast::SimpleFormat::new("name")),
                                  Box::new(ast::PlainText::new(" went to ")),
                                  Box::new(ast::SimpleFormat::new("place")),
                                  Box::new(ast::PlainText::new("."))]);
        let mut output = String::new();
        m.format_message(&mut output,
                            &arg("name", &"Jacob").arg("place", &"the store"))
            .unwrap();
        assert_eq!(output, "Jacob went to the store.".to_string());
    }
}
