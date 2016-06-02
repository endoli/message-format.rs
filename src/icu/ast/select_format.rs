// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::HashMap;
use std::fmt;

use {Args, Format, Message, Value};

/// Using a value, select the appropriate message and format it.
pub struct SelectFormat {
    /// The name of the variable whose value should be formatted.
    variable_name: String,
    /// Given a value of a variable, this maps that to a message format.
    mappings: HashMap<String, Message>,
    /// The message format to use if no valid mapping is found for
    /// the variable value.
    default: Message,
}

impl SelectFormat {
    /// Construct a `SelectFormat`.
    pub fn new(variable_name: &str, default: Message) -> Self {
        SelectFormat {
            variable_name: variable_name.to_string(),
            mappings: HashMap::<String, Message>::new(),
            default: default,
        }
    }

    /// Map a value for a particular message.
    pub fn map(&mut self, value: &str, message: Message) {
        self.mappings.insert(value.to_string(), message);
    }

    /// Given a value, determine which `Message` to use.
    pub fn lookup_message(&self, value: &str) -> &Message {
        self.mappings.get(value).unwrap_or(&self.default)
    }
}

impl Format for SelectFormat {
    fn apply_format<'f>(&'f self, stream: &mut fmt::Write, args: &'f Args<'f>) -> fmt::Result {
        if let Some(arg) = args.get(&self.variable_name) {
            let value = match *arg.value() {
                Value::Str(str) => str,
                _ => panic!("Wrong variable type."),
            };
            let message = self.lookup_message(&value);
            try!(message.write_message(stream, args));
            Ok(())
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use icu::parse;
    use super::SelectFormat;
    use {arg, Format};

    #[test]
    fn it_works() {
        let mut fmt = SelectFormat::new("type", parse("Default").unwrap());
        fmt.map("block", parse("Block").unwrap());

        let mut output = String::new();
        fmt.apply_format(&mut output, &arg("type", "block")).unwrap();
        assert_eq!("Block", output);

        let mut output = String::new();
        fmt.apply_format(&mut output, &arg("type", "span")).unwrap();
        assert_eq!("Default", output);
    }
}
