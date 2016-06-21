// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;

/// A wrapper around a value, used with [`Args`] so that a [`MessagePart`]
/// can access the original value when necessary.
///
/// These are not typically created directly, but are created automatically
/// via the standard library's `From` trait when creating [`Args`].
///
/// [`Args`]: struct.Args.html
/// [`MessagePart`]: trait.MessagePart.html
#[derive(Debug, PartialEq)]
pub enum Value<'a> {
    /// Wrap an `i64`.
    Number(i64),
    /// Wrap an `&str`.
    Str(&'a str),
}

impl<'a> From<i32> for Value<'a> {
    fn from(value: i32) -> Value<'a> {
        Value::Number(value as i64)
    }
}

impl<'a> From<u32> for Value<'a> {
    fn from(value: u32) -> Value<'a> {
        Value::Number(value as i64)
    }
}

impl<'a> From<i64> for Value<'a> {
    fn from(value: i64) -> Value<'a> {
        Value::Number(value)
    }
}

impl<'a> From<u64> for Value<'a> {
    fn from(value: u64) -> Value<'a> {
        Value::Number(value as i64)
    }
}

impl<'a> From<usize> for Value<'a> {
    fn from(value: usize) -> Value<'a> {
        Value::Number(value as i64)
    }
}

impl<'a> From<&'a str> for Value<'a> {
    fn from(value: &'a str) -> Value<'a> {
        Value::Str(value)
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
