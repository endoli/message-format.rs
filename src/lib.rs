// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Message Format
//!
//! This crate provides ICU-style message formatting. This provides
//! for formatting values taking localization rules into account.

#![warn(missing_docs)]
#![deny(trivial_numeric_casts,
        unsafe_code, unstable_features,
        unused_import_braces, unused_qualifications)]

use std::fmt;

pub mod ast;
mod parse_message;

use ast::{Format, MessagePart};
pub use self::parse_message::parse_message;

/// A message that has been localized and can be formatted in a
/// locale-aware manner.
pub struct Message {
    message_parts: Vec<MessagePart>,
}

impl Message {
    /// Construct a message from constituent parts.
    pub fn new(parts: Vec<MessagePart>) -> Self {
        Message { message_parts: parts }
    }

    /// Format a message to a stream.
    pub fn format_message(&self, stream: &mut fmt::Write, args: &Args) -> fmt::Result {
        for part in &self.message_parts {
            match *part {
                MessagePart::String(ref string) => {
                    try!(stream.write_str(string));
                }
                MessagePart::Placeholder => unreachable!(),
                MessagePart::Format(ref format) => try!(format.format_message_part(stream, args)),
            }
        }
        Ok(())
    }
}

struct Arg<'arg> {
    name: &'arg str,
    value: &'arg fmt::Display,
}

///
pub struct Args<'arg> {
    args: Vec<Arg<'arg>>,
}

impl<'arg> Args<'arg> {
    /// Construct new `Args`
    pub fn new() -> Self {
        Args { args: vec![] }
    }

    ///
    pub fn get(&self, name: &str) -> Option<&fmt::Display> {
        self.args.iter().find(|ref a| a.name == name).map(|a| a.value)
    }

    ///
    pub fn arg(mut self, name: &'arg str, value: &'arg fmt::Display) -> Self {
        self.args.push(Arg {
            name: name,
            value: value,
        });
        self
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
