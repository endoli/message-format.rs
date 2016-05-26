// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::error::Error;
use std::fmt;

use super::Message;

/// An error resulting from `parse_message`.
#[derive(Clone,Debug)]
pub enum ParseError {
    /// The message could not be parsed.
    NotImplemented,
}

impl Error for ParseError {
    fn description(&self) -> &str {
        match *self {
            ParseError::NotImplemented => "Not implemented.",
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.description().fmt(f)
    }
}

/// Parse some text and hopefully return a `Message`.
pub fn parse_message(_message: &str) -> Result<Message, ParseError> {
    Err(ParseError::NotImplemented)
}
