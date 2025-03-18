//! top-down parser using recursive descent

use std::iter::Peekable;
use crate::tokenizer::AbabaTokenizer;
use crate::{AbabaParseError, AbabaValue};

pub(crate) struct AbabaParser<'a> {
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
