// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// The set of [grammatical numbers] that we support.
///
/// These are used by the [ICU `PluralFormat`]. See also
/// [`english_cardinal_classifier`].
///
/// [grammatical numbers]: https://en.wikipedia.org/wiki/Grammatical_number
/// [ICU `PluralFormat`]: icu/ast/struct.PluralFormat.html
/// [`english_cardinal_classifier`]: fn.english_cardinal_classifier.html
#[derive(Clone,Copy,Debug,PartialEq)]
pub enum PluralCategory {
    /// Value is `0`.
    Zero,
    /// Value is `1`. In English, this corresponds to the "singular" form.
    One,
    /// Value is `2`.
    Two,
    /// Value is a few, more than `2`, but less than `many`. The exact
    /// range depends upon the locale.
    Few,
    /// Value is many, more than `few`. The exact range depends
    /// upon the locale.
    Many,
    /// Not one of the others. In English, this is used for the "plural"
    /// form.
    Other,
}
