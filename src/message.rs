// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;

use {Args, Format};

/// A message that has been localized and can be formatted in a
/// locale-aware manner.
///
/// While a `Message` can be created directly from [`Format`]
/// components, it is easiest to create it from [`icu::parse`].
///
/// A message can be formatted, returning a `String`:
///
/// ```
/// use message_format::*;
///
/// let m = icu::parse("{name} went to {place}.").unwrap();
/// assert_eq!(&m.format_message(&arg("name", "Jacob").arg("place", "the store")),
///            "Jacob went to the store.");
/// ```
///
/// It can also be written to a stream, but this is more cumbersome:
///
///
/// ```
/// use message_format::*;
///
/// let m = icu::parse("{name} went to {place}.").unwrap();
/// let mut output = String::new();
/// m.write_message(&mut output, &arg("name", "Jacob").arg("place", "the store")).unwrap();
/// assert_eq!(output, "Jacob went to the store.");
/// ```
///
/// [`Format`]: trait.Format.html
/// [`icu::parse`]: icu/fn.parse.html
pub struct Message {
    // This is pub due to icu::ast::PluralFormat. Once we address that, we
    // can make this private again.
    #[doc(hidden)]
    pub parts: Vec<Box<Format>>,
}

impl Message {
    /// Construct a message from constituent parts.
    pub fn new(parts: Vec<Box<Format>>) -> Self {
        Message { parts: parts }
    }

    /// Format a message, returning a string.
    pub fn format_message<'f>(&'f self, args: &'f Args<'f>) -> String {
        let mut output = String::new();
        let _ = self.write_message(&mut output, args);
        output
    }

    /// Write a message to a stream.
    pub fn write_message<'f>(&'f self, stream: &mut fmt::Write, args: &'f Args<'f>) -> fmt::Result {
        for part in &self.parts {
            try!(part.apply_format(stream, args));
        }
        Ok(())
    }
}
