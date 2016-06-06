// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;

use {Args, Context, MessagePart};

/// A placeholder for a value. Used by `PluralFormat`.
#[derive(Debug)]
pub struct PlaceholderFormat {
}

impl PlaceholderFormat {
    /// Construct a `PlaceholderFormat`.
    pub fn new() -> Self {
        PlaceholderFormat {}
    }
}

impl MessagePart for PlaceholderFormat {
    fn apply_format(&self,
                    context: &Context,
                    stream: &mut fmt::Write,
                    _args: &Args)
                    -> fmt::Result {
        if let Some(value) = context.placeholder_value {
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
    use {arg, Context, MessagePart};

    #[test]
    fn it_works() {
        let context = Context { placeholder_value: Some(3), ..Context::default() };
        let fmt = PlaceholderFormat::new();

        let mut output = String::new();
        fmt.apply_format(&context, &mut output, &arg("count", 0)).unwrap();
        assert_eq!("3", output);
    }
}
