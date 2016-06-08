// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(missing_docs)]

use std::error::Error;
use std::fmt;
use std::str;
use super::ast::*;

pub fn parse(source: &str) -> Result<Vec<Entry>, ParseError> {
    let mut p = Parser::new(source);
    p.parse()
}

#[derive(Debug)]
pub struct ParseError {
    pub error_message: String,
}

impl ParseError {
    pub fn new(error_message: &str) -> Self {
        ParseError { error_message: String::from(error_message) }
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        &self.error_message
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.description().fmt(f)
    }
}


struct Parser<'a> {
    source: str::Chars<'a>,
    ch: Option<char>,
    pos: u16,
}

impl<'a> Parser<'a> {
    fn new(source: &'a str) -> Parser<'a> {
        Parser {
            source: source.chars(),
            ch: None,
            pos: 0,
        }
    }

    fn bump(&mut self) {
        self.ch = self.source.next();

        self.pos += 1;
    }

    fn ch_is(&self, ch: char) -> bool {
        self.ch == Some(ch)
    }

    fn get_ws(&mut self) {
        while self.ch_is(' ') || self.ch_is('\n') || self.ch_is('\t') || self.ch_is('\r') {
            self.bump();
        }
    }

    fn get_line_ws(&mut self) {
        while self.ch_is(' ') || self.ch_is('\t') {
            self.bump();
        }
    }

    fn parse(&mut self) -> Result<Vec<Entry>, ParseError> {
        let mut entries: Vec<Entry> = Vec::new();

        self.get_ws();

        self.bump();

        loop {
            if self.ch == None {
                break;
            }

            let comment = None;
            match self.get_entry(comment) {
                Ok(entry) => entries.push(entry),
                Err(err) => return Err(err),
            }
            self.get_ws();
        }
        Ok(entries)
    }

    fn get_entry(&mut self, comment: Option<Comment>) -> Result<Entry, ParseError> {
        self.get_entity(comment)
    }

    fn get_entity(&mut self, comment: Option<Comment>) -> Result<Entry, ParseError> {
        let id = try!(self.get_identifier());
        self.get_line_ws();

        if !self.ch_is('=') {
            return Err(ParseError::new("Expected '='"));
        }
        self.bump();

        self.get_line_ws();

        match self.get_pattern() {
            Ok(value) => {
                Ok(Entry::Entity {
                    id: id,
                    comment: comment,
                    value: value,
                })
            }
            Err(err) => Err(err),
        }
    }

    fn get_identifier(&mut self) -> Result<Identifier, ParseError> {
        let mut name = String::new();

        let ch = match self.ch {
            Some(c) => c,
            None => return Err(ParseError::new("Unexpected end of input.")),
        };

        match ch {
            'a'...'z' | 'A'...'Z' | '_' => name.push(ch),
            _ => return Ok(Identifier { name: name }),
        }
        self.bump();

        while let Some(ch) = self.ch {
            match ch {
                'a'...'z' | 'A'...'Z' | '0'...'9' | '_' | '-' => name.push(ch),
                _ => break,
            }
            self.bump();
        }

        Ok(Identifier { name: name })
    }

    fn get_pattern(&mut self) -> Result<Value, ParseError> {
        let mut buffer = String::new();
        let mut source = String::new();
        let mut content = vec![];
        let mut quote_delimited: bool = false;
        let mut first_line = true;

        if self.ch_is('"') {
            quote_delimited = true;
        }

        loop {
            match self.ch {
                Some(c) if c == '\n' => {
                    if quote_delimited {
                        return Err(ParseError::new("Unclosed string"));
                    }
                    self.bump();
                    self.get_line_ws();

                    if !self.ch_is('|') {
                        break;
                    }
                    if first_line && !buffer.is_empty() {
                        return Err(ParseError::new("Multiline string should have the ID line \
                                                    empty"));
                    }
                    first_line = false;
                    self.bump();
                    if self.ch_is(' ') {
                        self.bump();
                    }
                    if !buffer.is_empty() {
                        buffer.push('\n');
                    }
                    continue;
                }
                Some(c) if c == '"' => {
                    self.bump();
                    quote_delimited = false;
                    break;
                }
                Some(c) => source.push(c),
                None => break,
            }
            match self.ch {
                Some(c) => buffer.push(c),
                None => continue,
            };
            self.bump();
        }

        if quote_delimited {
            return Err(ParseError::new("Unclosed string"));
        }

        if !buffer.is_empty() {
            // source.append(buffer);
            content.push(PatternElement::TextElement { value: source.clone() });
        }

        if content.is_empty() {
            // return Value::Pattern(source: source, elements: content);
        }

        content.push(PatternElement::TextElement { value: source.clone() });

        Ok(Value::Pattern {
            source: source,
            elements: content,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expected_parse(name: &str, source: &str) {
        match parse(source) {
            Err(e) => panic!("Parse failed: {}: {}", name, e),
            _ => {}
        }
    }

    fn expected_failure(name: &str, source: &str) {
        match parse(source) {
            Ok(_) => panic!("Parse unexpectedly worked: {}", name),
            _ => {}
        }
    }

    #[test]
    fn it_works() {
        expected_parse("simple", "a = b");
        expected_parse("simple", "a=b");
        expected_parse("simple", "a   =     b");
        expected_parse("multiline",
                       "multi =\n\
                        | abc\n\
                       ");
        expected_failure("comment", "#comment");
        expected_failure("comment", "# comment");
        expected_failure("comment", "#  comment");
    }
}
