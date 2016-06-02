// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// A wrapper around a value, used with `Arg` so that a `Format`
/// can access the original value when necessary.
pub enum Formattable<'a> {
    /// Wrap an `i64`.
    Number(i64),
    /// Wrap an `&str`.
    Str(&'a str),
    /// Wrap a `String`.
    String(&'a String),
}

/// Convert a value to a `Formattable` wrapper.
pub trait AsFormattable<'a> {
    /// Convert a value to a `Formattable` wrapper.
    fn as_formattable(&self) -> Formattable<'a>;
}

impl<'a> AsFormattable<'a> for &'a str {
    fn as_formattable(&self) -> Formattable<'a> {
        Formattable::Str(self)
    }
}

impl<'a> AsFormattable<'a> for i64 {
    fn as_formattable(&self) -> Formattable<'a> {
        Formattable::Number(*self)
    }
}
