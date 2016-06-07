// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(missing_docs)]

#[derive(Debug)]
pub enum Expression {
    IdentifierExpression {
        name: String,
    },
}

#[derive(Debug)]
pub enum PatternElement {
    TextElement {
        value: String,
    },
    Placeable {
        expressions: Vec<Expression>,
    },
}

#[derive(Debug)]
pub enum Value {
    Pattern {
        source: String,
        elements: Vec<PatternElement>,
    },
}

#[derive(Debug)]
pub enum Entry {
    Entity {
        id: Identifier,
        value: Value,
    },
}

#[derive(Debug)]
pub struct Identifier {
    pub name: String,
}
