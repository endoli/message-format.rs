// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;

use ast::Format;
use {Args, Message};

/// Using a value, select the appropriate message and format it.
pub struct SelectFormat<K> {
    /// The name of the variable whose value should be formatted.
    #[allow(dead_code)]
    variable_name: String,
    /// Given a value of a variable, this maps that to a message format.
    #[allow(dead_code)]
    mappings: HashMap<K, Message>,
    /// The message format to use if no valid mapping is found for
    /// the variable value.
    default: Message,
}

impl<K> SelectFormat<K>
    where K: Eq + Hash
{
    /// Construct a `SelectFormat`.
    pub fn new(variable_name: &str, default: Message) -> Self {
        SelectFormat {
            variable_name: variable_name.to_string(),
            mappings: HashMap::<K, Message>::new(),
            default: default,
        }
    }

    /// Map a value for a particular message.
    pub fn map(mut self, value: K, message: Message) -> Self {
        self.mappings.insert(value, message);
        self
    }
}

impl<K> Format for SelectFormat<K> {
    fn apply_format<'f>(&'f self, stream: &mut fmt::Write, args: &'f Args<'f>) -> fmt::Result {
        try!(self.default.format_message(stream, args));
        Ok(())
    }
}
