// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;

use {Args, Context, MessagePart, Message, Value};

#[derive(Debug)]
struct SelectMapping {
    value: String,
    message: Message,
}

/// Using a value, select the appropriate message and format it.
#[derive(Debug)]
pub struct SelectFormat {
    /// The name of the variable whose value should be formatted.
    variable_name: String,
    /// Given a value of a variable, this maps that to a message format.
    mappings: Vec<SelectMapping>,
    /// The message format to use if no valid mapping is found for
    /// the variable value.
    default: Message,
}

impl SelectFormat {
    /// Construct a `SelectFormat`.
    pub fn new(variable_name: &str, default: Message) -> Self {
        SelectFormat {
            variable_name: variable_name.to_string(),
            mappings: vec![],
            default: default,
        }
    }

    /// Map a value for a particular message.
    pub fn map(&mut self, value: &str, message: Message) {
        self.mappings.push(SelectMapping {
            value: value.to_string(),
            message: message,
        });
    }

    /// Given a value, determine which `Message` to use.
    pub fn lookup_message(&self, value: &str) -> &Message {
        self.mappings
            .iter()
            .find(|mapping| mapping.value == value)
            .map_or(&self.default, |mapping| &mapping.message)
    }
}

impl MessagePart for SelectFormat {
    fn apply_format<'f>(&self,
                        ctx: &Context,
                        stream: &mut fmt::Write,
                        args: Option<&Args<'f>>)
                        -> fmt::Result {
        let arg = args.and_then(|args| args.get(&self.variable_name));
        if let Some(&Value::Str(value)) = arg.map(|a| a.value()) {
            let message = self.lookup_message(value);
            try!(message.write_message(ctx, stream, args));
            Ok(())
        } else {
            Err(fmt::Error {})
        }
    }
}

#[cfg(test)]
mod tests {
    use icu::parse;
    use super::SelectFormat;
    use {Context, Message};

    #[test]
    fn it_works() {
        let ctx = Context::default();

        // Manually construct a message in an ugly way so that we aren't testing parsing.
        let mut fmt = SelectFormat::new("type", parse("Default").unwrap());
        fmt.map("block", parse("Block").unwrap());
        let msg = Message::new(vec![Box::new(fmt)]);

        let output = format_message!(ctx, &msg, type => "block");
        assert_eq!("Block", output);

        let output = format_message!(ctx, &msg, type => "span");
        assert_eq!("Default", output);
    }
}
