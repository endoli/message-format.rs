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

/// An error resulting from `parse`.
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

named!(variable_name <&str, &str>, is_not_s!(",}"));

named!(format <&str, Box<Format> >,
    delimited!(
        tag_s!("{"),
        chain!(
            name: variable_name,
            || Box::new(ast::SimpleFormat::new(name))),
        tag_s!("}")));

named!(plain_text <&str, Box<Format> >,
    map!(is_not_s!("{"), |text| Box::new(ast::PlainText::new(text))));

named!(message_parts <&str, Vec<Box<Format> > >,
    many0!(alt!(call!(format) | call!(plain_text))));

named!(pub message_parser <&str, Message>,
    chain!(
        parts: message_parts,
        || Message::new(parts)));

/// Parse some text and hopefully return a [`Message`].
///
/// [`Message`]: ../struct.Message.html
pub fn parse(message: &str) -> Result<Message, ParseError> {
    match message_parser(message) {
        IResult::Error(_) |
        IResult::Incomplete(_) => Err(ParseError::NotImplemented),
        IResult::Done(_, m) => Ok(m),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use arg;
    use nom::IResult;

    #[test]
    fn it_works() {
        match parse("{name} is from {city}.") {
            Ok(m) => {
                assert_eq!(m.format_message(&arg("name", "Hendrik").arg("city", "Berlin")),
                           "Hendrik is from Berlin.");
            }
            Err(e) => panic!("Parse failed: {}", e),
        }
    }

    #[test]
    fn incomplete_fails() {
        match message_parser("{name") {
            IResult::Incomplete(_) => {}
            IResult::Error(e) => panic!("Expected incomplete failure: Got {}", e),
            IResult::Done(_, _) => panic!("Expected incomplete failure, but succeeded."),
        }
    }

    #[test]
    fn all_text_works() {
        match message_parser("Hello, world!") {
            IResult::Done(_, _) => {}
            _ => panic!("Expected successful parse."),
        }
    }
}
