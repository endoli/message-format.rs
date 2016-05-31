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
//! MessageFormat is widely supported in many languages and environments.
//! This library will endeavor to support all of the ICU MessageFormat
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
mod parse_message;

use ast::Format;
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

#[allow(missing_docs)]
pub struct Arg<'a, T: 'a + fmt::Display + ?Sized> {
    name: &'a str,
    value: &'a T,
    prev: Option<&'a Args<'a>>,
}

#[allow(missing_docs)]
pub fn arg<'a, T: 'a + fmt::Display + ?Sized>(name: &'a str, value: &'a T) -> Arg<'a, T> {
    Arg {
        name: name,
        value: value,
        prev: None,
    }
}

#[allow(missing_docs)]
pub trait Args<'a> {
    fn arg<T: 'a + fmt::Display + ?Sized>(&'a self, name: &'a str, value: &'a T) -> Arg<'a, T>
        where Self: Sized
    {
        Arg {
            name: name,
            value: value,
            prev: Some(self),
        }
    }

    fn fmt_value(&self, f: &mut fmt::Formatter) -> fmt::Result;

    fn get(&'a self, name: &str) -> Option<&'a Args<'a>>;
}

impl<'a, T> Args<'a> for Arg<'a, T>
    where T: std::fmt::Display
{
    fn fmt_value(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.value.fmt(f)
    }

    fn get(&'a self, name: &str) -> Option<&'a Args<'a>> {
        if self.name == name {
            Some(self)
        } else if let Some(prev) = self.prev {
            prev.get(name)
        } else {
            None
        }
    }
}

impl<'a, 'b> fmt::Display for Args<'a> + 'b {
    /// Forward `fmt::Display` to the underlying value.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_value(f)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
