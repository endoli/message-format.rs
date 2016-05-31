// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Message Format AST
//!

use std::fmt;

mod placeholder_format;
mod plain_text;
mod plural_classifiers;
mod plural_format;
mod select_format;
mod simple_format;

pub use self::placeholder_format::PlaceholderFormat;
pub use self::plain_text::PlainText;
pub use self::plural_classifiers::*;
pub use self::plural_format::{PluralCategory, PluralFormat};
pub use self::select_format::SelectFormat;
pub use self::simple_format::SimpleFormat;

use super::Args;

/// The part of a message which formats a value.
pub trait Format {
    /// Format this message part.
    fn apply_format<'f>(&'f self, stream: &mut fmt::Write, args: &'f Args<'f>) -> fmt::Result;
}
