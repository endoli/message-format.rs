// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;

use {Args, Context, MessagePart};

/// A placeholder for a value. Used by `PluralFormat`.
#[derive(Debug, Default)]
pub struct PlaceholderFormat {}

impl PlaceholderFormat {
    /// Construct a `PlaceholderFormat`.
    pub fn new() -> Self {
        PlaceholderFormat::default()
    }
}

impl MessagePart for PlaceholderFormat {
    fn apply_format(
        &self,
        ctx: &Context,
        stream: &mut fmt::Write,
        _args: Option<&Args>,
    ) -> fmt::Result {
        if let Some(value) = ctx.placeholder_value {
            try!(write!(stream, "{}", value));
            Ok(())
        } else {
            Err(fmt::Error {})
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PlaceholderFormat;
    use {Context, Message};

    #[test]
    fn it_works() {
        let ctx = Context {
            placeholder_value: Some(3),
            ..Context::default()
        };

        let msg = Message::new(vec![Box::new(PlaceholderFormat::new())]);

        let output = format_message!(ctx, &msg);
        assert_eq!("3", output);
    }
}
