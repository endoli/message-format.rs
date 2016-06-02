// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::error::Error;
use std::fmt;
use std::str;

use nom::IResult;

use super::ast;
use {Format, Message};

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

named!(format_body <&[u8], Box<Format> >,
    map!(is_not!("}"), |name| Box::new(ast::SimpleFormat::new(str::from_utf8(name).unwrap()))));

named!(format <&[u8], Box<Format> >,
    delimited!(
        char!('{'),
        format_body,
        char!('}')));

named!(plain_text <&[u8], Box<Format> >,
    map!(is_not!("{"), |text| Box::new(ast::PlainText::new(str::from_utf8(text).unwrap()))));

named!(message_parts <&[u8], Vec<Box<Format> > >,
    many0!(alt!(call!(format) | call!(plain_text))));

named!(message_parser <&[u8], Message>,
    chain!(
        parts: message_parts,
        || Message::new(parts)));

/// Parse some text and hopefully return a [`Message`].
///
/// [`Message`]: ../struct.Message.html
pub fn parse_message(message: &str) -> Result<Message, ParseError> {
    match message_parser(message.as_bytes()) {
        IResult::Error(_) |
        IResult::Incomplete(_) => Err(ParseError::NotImplemented),
        IResult::Done(_, m) => Ok(m),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use arg;

    #[test]
    fn it_works() {
        match parse_message("{name} is from {city}.") {
            Ok(m) => {
                assert_eq!(m.format_message(&arg("name", "Hendrik").arg("city", "Berlin")),
                           "Hendrik is from Berlin.");
            }
            Err(e) => panic!("Parse failed: {}", e),
        }
    }
}
