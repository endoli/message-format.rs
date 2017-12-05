// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;

use {Args, Context, MessagePart};

/// A message that has been localized and can be formatted in a
/// locale-aware manner.
///
/// While a `Message` can be created directly from [`MessagePart`]
/// components, it is easiest to create it from [`icu::parse`].
///
/// [`MessagePart`]: trait.MessagePart.html
/// [`icu::parse`]: icu/fn.parse.html
#[derive(Debug)]
pub struct Message {
    parts: Vec<Box<MessagePart>>,
}

impl Message {
    /// Construct a message from constituent parts.
    pub fn new(parts: Vec<Box<MessagePart>>) -> Self {
        Message { parts: parts }
    }

    /// Write a message to a stream.
    ///
    /// This shouldn't be called directly in the usual case.
    /// Use `Context::write` or `Context::format` instead.
    pub fn write_message<'f>(
        &self,
        ctx: &Context,
        stream: &mut fmt::Write,
        args: Option<&Args<'f>>,
    ) -> fmt::Result {
        for part in &self.parts {
            try!(part.apply_format(ctx, stream, args));
        }
        Ok(())
    }
}
