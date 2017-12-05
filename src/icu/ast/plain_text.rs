// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;

use {Args, Context, MessagePart};

/// A string that should be output. Used for the text in between
/// formats.
#[derive(Debug)]
pub struct PlainText {
    /// The text that should be output.
    text: String,
}

impl PlainText {
    /// Construct a `PlainText`.
    pub fn new(text: &str) -> Self {
        PlainText { text: text.to_string() }
    }
}

impl MessagePart for PlainText {
    fn apply_format(
        &self,
        _ctx: &Context,
        stream: &mut fmt::Write,
        _args: Option<&Args>,
    ) -> fmt::Result {
        try!(stream.write_str(self.text.as_str()));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::PlainText;
    use {Context, Message};

    #[test]
    fn it_works() {
        let ctx = Context::default();

        let msg = Message::new(vec![Box::new(PlainText::new("Test text."))]);

        let output = format_message!(ctx, &msg);
        assert_eq!("Test text.", output);
    }
}
