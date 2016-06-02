// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;

use {Args, Format};

/// A string that should be output. Used for the text in between
/// formats.
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

impl Format for PlainText {
    fn apply_format(&self, stream: &mut fmt::Write, _args: &Args) -> fmt::Result {
        try!(stream.write_str(self.text.as_str()));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::PlainText;
    use {arg, Format};

    #[test]
    fn it_works() {
        let fmt = PlainText::new("Test text.");
        let mut output = String::new();
        fmt.apply_format(&mut output, &arg("John", "George")).unwrap();
        assert_eq!("Test text.", output);
    }
}
