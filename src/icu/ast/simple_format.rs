// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;

use {Args, Context, MessagePart};

/// A simple message consisting of a value to be formatted.
#[derive(Debug)]
pub struct SimpleFormat {
    /// The name of the variable whose value should be formatted.
    variable_name: String,
}

impl SimpleFormat {
    /// Construct a `SimpleFormat`.
    pub fn new(variable_name: &str) -> Self {
        SimpleFormat {
            variable_name: variable_name.to_string(),
        }
    }
}

impl MessagePart for SimpleFormat {
    fn apply_format<'f>(
        &self,
        _ctx: &Context,
        stream: &mut fmt::Write,
        args: Option<&Args<'f>>,
    ) -> fmt::Result {
        if let Some(arg) = args.and_then(|args| args.get(&self.variable_name)) {
            try!(write!(stream, "{}", arg.value()));
            Ok(())
        } else {
            Err(fmt::Error {})
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SimpleFormat;
    use {Context, Message};

    #[test]
    fn it_works() {
        let ctx = Context::default();

        // Manually construct a message in an ugly way so that we aren't testing parsing.
        let fmt = SimpleFormat::new("name");
        let msg = Message::new(vec![Box::new(fmt)]);

        let output = format_message!(ctx, &msg, name => "John");
        assert_eq!("John", output);
    }
}
