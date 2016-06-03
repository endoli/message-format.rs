// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;

use {Args, MessagePart};

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
    fn apply_format(&self, _stream: &mut fmt::Write, _args: &Args) -> fmt::Result {
        unimplemented!();
    }
}
