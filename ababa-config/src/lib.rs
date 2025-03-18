//! ababa config language (my own)
//!
//! this grammar is LL(1).
//!
//! the input is parsed into tokens using [AbabaTokenizer] before being parsed by [AbabaParser]
//! into an AST of [AbabaValue] using recursive descent.
//! this is implemented via the [TryFrom<String>] trait.
//!
//! the [TryFrom<AbabaValue>] trait is implemented for some types, such as all number types and vectors.
//! please do use it when creating your own impls for your own types.
//!
//! you can use the `#[derive(AbabaDeserialize)]` macro (implemented in [ababa_config_proc]) to
//! derive [TryFrom<String>] for your own structs. no guarantees though.
//!
//! ## grammar
//!
//! ```txt
//! config         ::= list | tuple | object | number
//!
//! list           ::= '[' list-args
//! list-args      ::= config ',' list-args | ']'
//!
//! tuple          ::= '('
//! tuple-args      ::= config ',' tuple-args| ')'
//!
//! object         ::= ident '{' object-fields | '{' object-fields
//! object-fields  ::= ident ':' config ',' object-fields | '}'
//!
//! ident          ::= `[a-zA-Z_][0-9a-zA-Z-_]+`
//!
//! number         ::= whatever rust uses honestly
//! ```
//!
//! - expressions using backticks are to be interpreted as regex
//! - objects can optionally specify a type, but can leave it out
use crate::parser::AbabaParser;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};

pub mod impls;

mod parser;
mod tokenizer;

#[derive(Debug)]
pub enum AbabaParseError {
    NumberOutOfBounds {
        x: f64,
        target_type: &'static str,
    },
    ValueTypeDidNotMatch {
        expected: &'static str,
        got: AbabaValue,
    },
    StructTypeDidNotMatch {
        expected: &'static str,
        got: Option<String>,
    },
    StructFieldNotPresent {
        field: &'static str,
    },
    NoContent,
    MissingEndBrace {
        brace: char,
    },
    NotEnoughElements {
        expected: i32,
        got: usize,
    },
}

impl Display for AbabaParseError {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for AbabaParseError {}

#[derive(Debug, Clone)]
pub enum AbabaValue {
    Number(f64),
    Object {
        struct_type: Option<String>,
        fields: HashMap<String, AbabaValue>,
    },
    List(Vec<AbabaValue>),
    Tuple(Vec<AbabaValue>),
}

impl TryFrom<String> for AbabaValue {
    type Error = AbabaParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        AbabaParser::new(&value).parse()
    }
}
