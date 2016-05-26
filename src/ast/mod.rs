// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Message Format AST
//!

use std::fmt;

mod plural_classifiers;
mod plural_format;
mod select_format;
mod simple_format;

pub use self::plural_classifiers::*;
pub use self::plural_format::{PluralCategory, PluralFormat};
pub use self::select_format::SelectFormat;
pub use self::simple_format::SimpleFormat;

use super::Args;

/// The part of a message which formats a value.
pub trait Format {
    /// Format this message part.
    fn format_message_part(&self, stream: &mut fmt::Write, args: &Args) -> fmt::Result;
}

/// Either some plain text (string)
/// or something to be formatted.
pub enum MessagePart {
    /// A message part which is a piece of plain text
    /// that needs no formatting.
    String(String),
    /// Magic value used internally by some formats. Currently
    /// only used for `PluralFormat`.
    Placeholder,
    /// A message part which needs to be formatted.
    Format(Box<Format>),
}
