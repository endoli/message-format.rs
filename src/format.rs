// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;

use Args;

/// The part of a message which formats a value.
pub trait Format: fmt::Debug {
    /// Format this message part.
    fn apply_format<'f>(&'f self, stream: &mut fmt::Write, args: &'f Args<'f>) -> fmt::Result;
}
