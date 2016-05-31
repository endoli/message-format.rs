// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::HashMap;
use std::fmt;

use super::Format;
use super::english_cardinal_classifier;
use {Args, Message};

/// The set of [grammatical numbers] that we support.
///
/// [grammatical numbers]: https://en.wikipedia.org/wiki/Grammatical_number
#[derive(Clone,Copy,Debug,PartialEq)]
pub enum PluralCategory {
    /// Value is `0`.
    Zero,
    /// Value is `1`. In English, this corresponds to the "singular" form.
    One,
    /// Value is `2`.
    Two,
    /// Value is a few, more than `2`, but less than `many`. The exact
    /// range depends upon the locale.
    Few,
    /// Value is many, more than `few`. The exact range depends
    /// upon the locale.
    Many,
    /// Not one of the others. In English, this is used for the "plural"
    /// form.
    Other,
}

/// Format a value taking pluralization rules into account.
pub struct PluralFormat {
    /// The name of the variable whose value should be formatted.
    #[allow(dead_code)]
    variable_name: String,
    classifier: fn(i64) -> PluralCategory,
    literals: HashMap<i64, Message>,
    offset: i64,
    zero: Option<Message>,
    one: Option<Message>,
    two: Option<Message>,
    few: Option<Message>,
    many: Option<Message>,
    other: Message,
}

impl PluralFormat {
    /// Construct a `PluralFormat`.
    pub fn new(variable_name: &str, other: Message) -> Self {
        PluralFormat {
            variable_name: variable_name.to_string(),
            classifier: english_cardinal_classifier,
            literals: HashMap::new(),
            offset: 0,
            zero: None,
            one: None,
            two: None,
            few: None,
            many: None,
            other: other,
        }
    }

    /// Set the `message` to be used for a literal value.
    pub fn literal(mut self, literal: i64, message: Message) -> Self {
        self.literals.insert(literal, message);
        self
    }

    /// Apply an `offset`.
    pub fn offset(mut self, offset: i64) -> Self {
        self.offset = offset;
        self
    }

    /// Set the `message` for `PluralCategory::Zero`.
    pub fn zero(mut self, message: Message) -> Self {
        self.zero = Some(message);
        self
    }

    /// Set the `message` for `PluralCategory::One`.
    pub fn one(mut self, message: Message) -> Self {
        self.one = Some(message);
        self
    }

    /// Set the `message` for `PluralCategory::Two`.
    pub fn two(mut self, message: Message) -> Self {
        self.two = Some(message);
        self
    }

    /// Set the `message` for `PluralCategory::Few`.
    pub fn few(mut self, message: Message) -> Self {
        self.few = Some(message);
        self
    }

    /// Set the `message` for `PluralCategory::Many`.
    pub fn many(mut self, message: Message) -> Self {
        self.many = Some(message);
        self
    }

    /// Given a value adjusted by the `offset`, determine which `Message` to use.
    fn lookup_message(&self, offset_value: i64) -> &Message {
        if let Some(literal) = self.literals.get(&offset_value) {
            literal
        } else {
            let category = (self.classifier)(offset_value);
            match category {
                PluralCategory::Zero => self.zero.as_ref().unwrap_or(&self.other),
                PluralCategory::One => self.one.as_ref().unwrap_or(&self.other),
                PluralCategory::Two => self.two.as_ref().unwrap_or(&self.other),
                PluralCategory::Few => self.few.as_ref().unwrap_or(&self.other),
                PluralCategory::Many => self.many.as_ref().unwrap_or(&self.other),
                PluralCategory::Other => &self.other,
            }
        }
    }

    /// Handle specialized behavior for `PlaceholderFormat` when formatting
    /// a `PluralFormat`.
    ///
    /// XXX: Can we fold this away and do something more general for
    /// `PlaceholderFormat`?
    fn format_plural_message<'f>(&'f self,
                                 stream: &mut fmt::Write,
                                 message: &'f Message,
                                 _offset_value: i64,
                                 args: &'f Args<'f>)
                                 -> fmt::Result {
        for part in &message.parts {
            // XXX: Need to deal with PlaceholderFormat here and give it
            // the `offset_value`.
            try!(part.apply_format(stream, args));
        }
        Ok(())
    }
}

impl Format for PluralFormat {
    fn apply_format<'f>(&'f self, stream: &mut fmt::Write, args: &'f Args<'f>) -> fmt::Result {
        let value = 0;
        let offset_value = value - self.offset;
        let message = self.lookup_message(offset_value);
        try!(self.format_plural_message(stream, message, offset_value, args));
        Ok(())
    }
}
