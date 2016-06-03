// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::PluralCategory;

/// English cardinal plural classifier.
///
/// In English, a single item is mapped to `PluralCategory::One`,
/// with all other numbers mapped to `PluralCategory::Other`.
///
/// ```
/// use message_format::*;
///
/// assert_eq!(english_cardinal_classifier(0), PluralCategory::Other);
/// assert_eq!(english_cardinal_classifier(1), PluralCategory::One);
/// assert_eq!(english_cardinal_classifier(2), PluralCategory::Other);
/// ```
pub fn english_cardinal_classifier(value: i64) -> PluralCategory {
    match value {
        1 => PluralCategory::One,
        _ => PluralCategory::Other,
    }
}
