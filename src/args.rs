// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::{AsValue, Value};

/// Holds the arguments being used to format a [`Message`].
///
/// This is a linked list. This avoids any allocations for a `Vec`
/// or `HashMap`. There won't be enough arguments to most messages
/// to make doing linear searches on the arguments costly enough
/// to matter.
///
/// [`Message`]: struct.Message.html
pub struct Args<'a> {
    name: &'a str,
    value: Value<'a>,
    prev: Option<&'a Args<'a>>,
}

/// Create an argument holder.
///
/// ```
/// use message_format::arg;
///
/// let args = arg("name", "John");
/// assert!(args.get("name").is_some());
/// ```
pub fn arg<'a, T: 'a + AsValue<'a>>(name: &'a str, value: T) -> Args<'a> {
    Args {
        name: name,
        value: value.as_formattable(),
        prev: None,
    }
}

impl<'a> Args<'a> {
    /// Add an additional argument. This returns a new value which maintains a link
    /// to the old value. You must maintain a reference to the return value for it to
    /// remain valid.
    ///
    /// ```
    /// use message_format::arg;
    ///
    /// let args = arg("name", "John");
    /// let args = args.arg("city", "Rome");
    /// assert!(args.get("name").is_some());
    /// assert!(args.get("city").is_some());
    /// ```
    pub fn arg<T: 'a + AsValue<'a>>(&'a self, name: &'a str, value: T) -> Args<'a>
        where Self: Sized
    {
        Args {
            name: name,
            value: value.as_formattable(),
            prev: Some(self),
        }
    }

    /// Retrieve the argument with the given `name`.
    ///
    /// ```
    /// use message_format::arg;
    ///
    /// let args = arg("count", 3);
    /// let arg = args.get("count").unwrap();
    /// ```
    pub fn get(&'a self, name: &str) -> Option<&'a Args<'a>> {
        if self.name == name {
            Some(self)
        } else if let Some(prev) = self.prev {
            prev.get(name)
        } else {
            None
        }
    }

    /// Retrieve the [`Value`] wrapper around the argument value.
    ///
    /// ```
    /// use message_format::{arg, Value};
    ///
    /// let args = arg("count", 3);
    /// let arg = args.get("count").unwrap();
    /// if let &Value::Number(count) = arg.value() {
    ///     assert_eq!(count, 3);
    /// } else {
    ///     panic!("The count was not a number!");
    /// }
    /// ```
    ///
    /// [`Value`]: enum.Value.html
    pub fn value(&'a self) -> &'a Value<'a> {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::Value;

    #[test]
    fn get_works() {
        let name = "John";
        let args = arg("name", name);
        assert_eq!(format!("{}", args.get("name").unwrap().value()), "John");
    }

    #[test]
    fn numbers_work() {
        let count = 3;
        let args = arg("count", count);
        assert_eq!(args.get("count").unwrap().value(), &Value::Number(3));
        assert_eq!(format!("{}", args.get("count").unwrap().value()), "3");
    }
}
