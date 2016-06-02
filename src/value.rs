// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;

/// A wrapper around a value, used with `Arg` so that a `Format`
/// can access the original value when necessary.
#[derive(Debug,PartialEq)]
pub enum Value<'a> {
    /// Wrap an `i64`.
    Number(i64),
    /// Wrap an `&str`.
    Str(&'a str),
}

/// Convert a value to a `Value` wrapper.
pub trait AsValue<'a> {
    /// Convert a value to a `Value` wrapper.
    fn as_formattable(&self) -> Value<'a>;
}

impl<'a> AsValue<'a> for i64 {
    fn as_formattable(&self) -> Value<'a> {
        Value::Number(*self)
    }
}

impl<'a> AsValue<'a> for &'a str {
    fn as_formattable(&self) -> Value<'a> {
        Value::Str(self)
    }
}

impl<'a> fmt::Display for Value<'a> {
    /// Forward `fmt::Display` to the underlying value.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Number(i) => i.fmt(f),
            Value::Str(s) => s.fmt(f),
        }
    }
}
