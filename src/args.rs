// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::Value;

/// Holds the arguments being used to format a [`Message`].
///
/// This is a linked list. This avoids any allocations for a `Vec`
/// or `HashMap`. There won't be enough arguments to most messages
/// to make doing linear searches on the arguments costly enough
/// to matter.
///
/// [`Message`]: struct.Message.html
pub struct Args<'a> {
    /// The name of the argument which must match the usage within
    /// the message text.
    pub name: &'a str,
    /// The value of the argument.
    pub value: Value<'a>,
    /// The 'next' argument (which is really the previous since this
    /// is a linked list with the last argument first).
    pub prev: Option<&'a Args<'a>>,
}

/// Create an argument holder.
///
/// This isn't commonly used as arguments are usually set up via the
/// `format_message!` or `write_message!` macros.
///
/// ```
/// use message_format::arg;
///
/// let args = arg("name", "John");
/// assert!(args.get("name").is_some());
/// ```
pub fn arg<'a, T: 'a>(name: &'a str, value: T) -> Args<'a>
where
    Value<'a>: From<T>,
{
    Args {
        name: name,
        value: Value::from(value),
        prev: None,
    }
}

impl<'a> Args<'a> {
    /// Add an additional argument. This returns a new value which maintains a link
    /// to the old value. You must maintain a reference to the return value for it to
    /// remain valid.
    ///
    /// This isn't commonly used as arguments are usually set up via the
    /// `format_message!` or `write_message!` macros.
    ///
    /// ```
    /// use message_format::arg;
    ///
    /// let args = arg("name", "John");
    /// let args = args.arg("city", "Rome");
    /// assert!(args.get("name").is_some());
    /// assert!(args.get("city").is_some());
    /// ```
    pub fn arg<T: 'a>(&'a self, name: &'a str, value: T) -> Args<'a>
    where
        Self: Sized,
        Value<'a>: From<T>,
    {
        Args {
            name: name,
            value: Value::from(value),
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
