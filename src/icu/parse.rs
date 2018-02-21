// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::error::Error;
use std::fmt;
use std::str;

use nom::{multispace, IResult};

use super::ast;
use {Message, MessagePart};

/// An error resulting from `parse`.
#[derive(Clone, Debug)]
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

/// Given a name, create a `SimpleFormat`.
fn mk_simple(name: &str) -> Box<MessagePart> {
    Box::new(ast::SimpleFormat::new(name))
}

/// This grabs the variable name from a format, which is
/// the first thing after the '{' and extends to the first
/// ',' or '}'.
///
/// '{name}' has a variable name of 'name'.
named!(variable_name <&str, &str>, is_not_s!(",}"));

/// A simple format has only a name, delimited by braces.
named!(simple_format <&str, Box<MessagePart> >,
    map!(
        delimited!(
            tag_s!("{"),
            variable_name,
            tag_s!("}")),
        mk_simple));

named!(plural_format <&str, Box<MessagePart> >,
    delimited!(
        tag_s!("{"),
        do_parse!(
            name: variable_name >>
            tag_s!(",") >> opt!(multispace) >>
            tag_s!("plural") >> opt!(multispace) >>
            (Box::new(ast::SimpleFormat::new(name)))),
        tag_s!("}")));

named!(select_format <&str, Box<MessagePart> >,
    delimited!(
        tag_s!("{"),
        do_parse!(
            name: variable_name >>
            tag_s!(",") >> opt!(multispace) >>
            tag_s!("select") >> opt!(multispace) >>
            (Box::new(ast::SimpleFormat::new(name)))),
        tag_s!("}")));

/// Plain text extends up through to the start of the next format
/// block.
named!(plain_text <&str, Box<MessagePart> >,
    map!(is_not_s!("{"), |text| Box::new(ast::PlainText::new(text))));

/// Message parts must be 1 of the various part types. And there must
/// be at least one of them for now.
named!(message_parts <&str, Vec<Box<MessagePart> > >,
    many1!(
        alt!(call!(simple_format) |
             call!(plural_format) |
             call!(select_format) |
             call!(plain_text))));

/// Given a set of `MessagePart`s, create a `Message`.
named!(pub message_parser <&str, Message>,
    map!(message_parts, Message::new));

/// Parse some text and hopefully return a [`Message`].
///
/// [`Message`]: ../struct.Message.html
pub fn parse(message: &str) -> Result<Message, ParseError> {
    match message_parser(message) {
        IResult::Error(_) | IResult::Incomplete(_) => Err(ParseError::NotImplemented),
        IResult::Done(_, m) => Ok(m),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use {arg, Context};
    use nom::IResult;

    #[test]
    fn it_works() {
        let ctx = Context::default();
        match parse("{name} is from {city}.") {
            Ok(m) => {
                assert_eq!(
                    ctx.format(&m, Some(&arg("name", "Hendrik").arg("city", "Berlin"))),
                    "Hendrik is from Berlin."
                );
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

    #[test]
    fn plural_format_works() {
        match message_parser("{count,plural}") {
            IResult::Done(_, _) => {}
            _ => panic!("Expected successful parse."),
        }
    }

    #[test]
    fn select_format_works() {
        match message_parser("{type,select}") {
            IResult::Done(_, _) => {}
            _ => panic!("Expected successful parse."),
        }
    }
}
