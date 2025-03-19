//! top-down parser using recursive descent

use crate::tokenizer::AbabaTokenizer;
use crate::{AbabaParseError, AbabaValue};
use std::iter::Peekable;

pub struct AbabaParser<'a> {
    _tokenizer: Peekable<AbabaTokenizer<'a>>,
}

impl<'a> AbabaParser<'a> {
    pub fn new(s: &'a str) -> Self {
        AbabaParser {
            _tokenizer: AbabaTokenizer::new(s).peekable(),
        }
    }

    pub fn parse(self) -> Result<AbabaValue, AbabaParseError> {
        todo!()
    }
}
