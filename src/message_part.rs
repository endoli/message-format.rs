// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;

use {Args, Context};

/// Part of a message. May be something that requires formatting a
/// value or just plain text.
pub trait MessagePart: fmt::Debug {
    /// Format this message part.
    fn apply_format<'f>(&'f self,
                        context: &Context,
                        stream: &mut fmt::Write,
                        args: &'f Args<'f>)
                        -> fmt::Result;
}
