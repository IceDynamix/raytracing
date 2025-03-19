//! ababa config language (my own)
//!
//! the input is parsed into tokens using [AbabaTokenizer], then parsed into an AST
//! of [AbabaValue] with [AbabaParser].
//!
//! ## grammar
//!
//! this grammar is LL(1) and is parsed using recursive descent.
//!
//! - all whitespace is ignored
//! - ε represents the empty string
//! - text in single quotes represent literals, no quotes represent variables
//! - expressions using backticks are to be interpreted as regex (although it's not used in this
//!   project to stay true to the "no dependency" rule)
//! - objects can optionally specify a type (ident), but it can be left out
//! - idents do not start with a minus or a digit in order to differentiate from a number
//! - lists, tuples and objects have optional trailing commas
//! 
//! ```txt
//! value       ::= list | tuple | object | number
//!
//! list        ::= '[' items ']'
//! tuple       ::= '(' items ')'
//!
//! items       ::= value item-cont
//! items-cont  ::= ',' item | ',' | ε
//!
//! object      ::= ident '{' fields '}'
//! fields      ::= ident ':' value field-cont
//! fields-cont ::= ',' field | ',' | ε
//! 
//! ident       ::= `[a-zA-Z_][0-9a-zA-Z-_]+`
//! number      ::= whatever rust uses honestly
//! ```
//!
//! ## implementing your own deserialization
//!
//! the [TryFrom]<AbabaValue> trait is implemented for some types, such as all number types and vectors.
//! please do use it when creating your own impls for your own types.
//!
//! you can use the `#[derive(AbabaDeserialize)]` macro (implemented in [ababa_config_proc]) to
//! derive [TryFrom]<String> for your own structs. no guarantees though.
//!
//! ```
//! use ababa_config::{AbabaParseError, AbabaValue};
//!
//! pub struct Vector3 {
//!     pub x: f64,
//!     pub y: f64,
//!     pub z: f64,
//! }
//!
//! impl TryFrom<AbabaValue> for Vector3 {
//!     type Error = AbabaParseError;
//!
//!     fn try_from(value: AbabaValue) -> Result<Self, Self::Error> {
//!         let (x, y, z) = value.try_into()?; // uses the TryFrom implementation of (f64, f64, f64)
//!         Ok(Vector3 { x, y, z })
//!     }
//! }
//! ```
//!
//! ```
//! use ababa_config::{AbabaParser, AbabaValue};
//! use ababa_config_proc::AbabaDeserialize;
//!
//! // generates TryFrom<AbabaValue> implementation!
//! #[derive(AbabaDeserialize)]
//! pub struct Ababa {
//!     a: f64,
//!     ba: i64,
//! }
//!
//! // you can use the TryFrom<String> trait of AbabaValue, or use AbabaParser directly
//! let s = "Ababa { a: 5.0, ba: 64 }".to_string();
//! let a: Ababa = AbabaParser::new(&s).parse()?.try_into()?;
//! ```
pub use crate::parser::AbabaParser;
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
