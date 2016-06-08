// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_export]
macro_rules! format_message {
    ($ctx:expr, $msg:expr) => {
        $ctx.format($msg, None)
    };
    ($ctx:expr, $msg:expr, $($rest:tt)*) => ({
        $ctx.format($msg, message_args!($($rest)*))
    });
}

#[macro_export]
macro_rules! write_message {
    ($ctx:expr, $msg:expr, $stream:expr) => {
        $ctx.write($msg, $stream, None)
    };
    ($ctx:expr, $msg:expr, $stream:expr, $($rest:tt)*) => ({
        $ctx.write($msg, $stream, message_args!($($rest)*))
    });
}

#[macro_export]
macro_rules! message_args_aux {
    ($prev:expr, $name:ident => $value:expr) => {
        Some(&Args {
            name: stringify!($name),
            value: $value.as_formattable(),
            prev: $prev,
        })
    };
    ($prev:expr, $name:ident) => {
        Some(&Args {
            name: stringify!($name),
            value: $name.as_formattable(),
            prev: $prev,
        })
    };
    ($prev:expr, $name:ident, $($rest:tt)*) => {
        message_args_aux!(
            Some(&Args {
                name: stringify!($name),
                value: $name.as_formattable(),
                prev: $prev,
            }),
            $($rest)*)
    };
    ($prev:expr, $name:ident => $value:expr, $($rest:tt)*) => {
        message_args_aux!(
            Some(&Args {
                name: stringify!($name),
                value: $value.as_formattable(),
                prev: $prev,
            }),
            $($rest)*)
    };
}

#[macro_export]
macro_rules! message_args {
    () => { None };
    ($name:ident => $value:expr) => {
        Some(&Args {
            name: stringify!($name),
            value: $value.as_formattable(),
            prev: None,
        })
    };
    ($name:ident) => {
        Some(&Args {
            name: stringify!($name),
            value: $name.as_formattable(),
            prev: None,
        })
    };
    ($name:ident, $($rest:tt)*) => {
        message_args_aux!(
            Some(&Args {
                name: stringify!($name),
                value: $name.as_formattable(),
                prev: None,
            }),
            $($rest)*)
    };
    ($name:ident => $value:expr, $($rest:tt)*) => {
        message_args_aux!(
            Some(&Args {
                name: stringify!($name),
                value: $value.as_formattable(),
                prev: None,
            }),
            $($rest)*)
    };
}

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn format_without_args() {
        let ctx = Context::default();

        let m = icu::parse("Hello!").unwrap();
        let s = format_message!(ctx, &m);
        assert_eq!(s, "Hello!");
    }

    #[test]
    fn format_single_arg() {
        let ctx = Context::default();

        let m = icu::parse("{name}").unwrap();
        let name = "John";
        let s = format_message!(ctx, &m, name);
        assert_eq!(s, "John");
    }

    #[test]
    fn format_single_named_arg() {
        let ctx = Context::default();

        let m = icu::parse("{name}").unwrap();
        let s = format_message!(ctx, &m, name => "John");
        assert_eq!(s, "John");
    }

    #[test]
    fn format_two_args() {
        let ctx = Context::default();

        let m = icu::parse("{a}{b}").unwrap();
        let b = "2";
        let s = format_message!(ctx, &m, a => "1", b);
        assert_eq!(s, "12");
    }

    #[test]
    fn format_three_args() {
        let ctx = Context::default();

        let m = icu::parse("{a}{c}{b}").unwrap();
        let s = format_message!(ctx, &m, a => "1", b => "2", c => "3");
        assert_eq!(s, "132");
    }

    #[test]
    fn write_without_args() {
        let ctx = Context::default();

        let m = icu::parse("Hello!").unwrap();
        let mut stream = String::new();
        write_message!(ctx, &m, &mut stream).unwrap();
        assert_eq!(stream, "Hello!");
    }

    #[test]
    fn write_single_arg() {
        let ctx = Context::default();

        let m = icu::parse("{name}").unwrap();
        let mut stream = String::new();
        write_message!(ctx, &m, &mut stream, name => "John").unwrap();
        assert_eq!(stream, "John");
    }
}
