// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use language_tags::LanguageTag;

/// Contextual configuration data.
#[derive(Clone)]
pub struct Context {
    /// The language being localized for.
    pub language_tag: LanguageTag,
    /// The value to use in a `PlaceholderFormat`.
    pub placeholder_value: Option<i64>,
}

impl Context {
    /// Create a new instance of `Context`.
    pub fn new(language: LanguageTag, placeholder_value: Option<i64>) -> Self {
        Context {
            language_tag: language,
            placeholder_value: placeholder_value,
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Context {
            language_tag: Default::default(),
            placeholder_value: None,
        }
    }
}
