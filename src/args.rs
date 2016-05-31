// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;

#[allow(missing_docs)]
pub struct Arg<'a, T: 'a + fmt::Display + ?Sized> {
    name: &'a str,
    value: &'a T,
    prev: Option<&'a Args<'a>>,
}

#[allow(missing_docs)]
pub fn arg<'a, T: 'a + fmt::Display + ?Sized>(name: &'a str, value: &'a T) -> Arg<'a, T> {
    Arg {
        name: name,
        value: value,
        prev: None,
    }
}

#[allow(missing_docs)]
pub trait Args<'a> {
    fn arg<T: 'a + fmt::Display + ?Sized>(&'a self, name: &'a str, value: &'a T) -> Arg<'a, T>
        where Self: Sized
    {
        Arg {
            name: name,
            value: value,
            prev: Some(self),
        }
    }

    fn fmt_value(&self, f: &mut fmt::Formatter) -> fmt::Result;

    fn get(&'a self, name: &str) -> Option<&'a Args<'a>>;
}

impl<'a, T> Args<'a> for Arg<'a, T>
    where T: fmt::Display
{
    fn fmt_value(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.value.fmt(f)
    }

    fn get(&'a self, name: &str) -> Option<&'a Args<'a>> {
        if self.name == name {
            Some(self)
        } else if let Some(prev) = self.prev {
            prev.get(name)
        } else {
            None
        }
    }
}

impl<'a, 'b> fmt::Display for Args<'a> + 'b {
    /// Forward `fmt::Display` to the underlying value.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_value(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fmt_works() {
        let name = "George";
        let args: &Args = &arg("name", &name);
        assert_eq!(format!("{}", args), "George");
    }

    #[test]
    fn get_works() {
        let name = "John";
        let args: &Args = &arg("name", &name);
        assert_eq!(format!("{}", args.get("name").unwrap()), "John");
    }

    #[test]
    fn numbers_work() {
        let count = 3;
        let args: &Args = &arg("count", &count);
        assert_eq!(format!("{}", args.get("count").unwrap()), "3");
    }
}
