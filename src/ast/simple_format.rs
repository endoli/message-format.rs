// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;

use super::Format;
use Args;

/// A simple message consisting of a value to be formatted.
pub struct SimpleFormat {
    /// The name of the variable whose value should be formatted.
    variable_name: String,
}

impl SimpleFormat {
    /// Construct a `SimpleFormat`.
    pub fn new(variable_name: &str) -> Self {
        SimpleFormat { variable_name: variable_name.to_string() }
    }
}

impl Format for SimpleFormat {
    fn apply_format(&self, stream: &mut fmt::Write, args: &Args) -> fmt::Result {
        if let Some(value) = args.get(self.variable_name.as_str()) {
            try!(write!(stream, "{}", value));
            Ok(())
        } else {
            // XXX: Should we return an error in this case?
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast::Format;
    use arg;

    #[test]
    fn it_works() {
        let fmt = SimpleFormat::new("name");
        let mut output = String::new();
        fmt.apply_format(&mut output, &arg("name", &"John")).unwrap();
        assert_eq!("", output);
    }
}
