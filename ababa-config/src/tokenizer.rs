use std::iter::Peekable;
use std::str::CharIndices;

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum AbabaToken<'a> {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    LeftParen,
    RightParen,
    ListSeparator,  // comma
    FieldSeparator, // colon
    Ident(&'a str),
    Number(&'a str),
    UnknownChar(char),
}

pub(crate) struct AbabaTokenizer<'a> {
    s: &'a str,
    inner: Peekable<CharIndices<'a>>,
}

impl<'a> AbabaTokenizer<'a> {
    pub fn new(s: &'a str) -> Self {
        AbabaTokenizer {
            s,
            inner: s.char_indices().peekable(),
        }
    }
}

impl<'a> Iterator for AbabaTokenizer<'a> {
    type Item = AbabaToken<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        // ignore whitespace!
        // basically just an awkward take_while() that uses peeking
        while self.inner.next_if(|(_, c)| c.is_whitespace()).is_some() {}

        use AbabaToken::*;
        let tok = match self.inner.next()? {
            (_, '{') => LeftBrace,
            (_, '}') => RightBrace,
            (_, '[') => LeftBracket,
            (_, ']') => RightBracket,
            (_, '(') => LeftParen,
            (_, ')') => RightParen,
            (_, ',') => ListSeparator,
            (_, ':') => FieldSeparator,
            (i, '-' | '0'..='9') => {
                let start = i;

                while self
                    .inner
                    .next_if(|(_, c)| matches!(c, '0'..='9' | '.' | '-' | '_'))
                    .is_some()
                {}

                let end = *self.inner.peek().map(|(i, _)| i).unwrap_or(&self.s.len());

                Number(&self.s[start..end])
            }
            (i, 'a'..='z' | 'A'..='Z' | '_') => {
                let start = i;

                while self
                    .inner
                    .next_if(|(_, c)| matches!(c, 'a'..='z' | 'A'..='Z' | '_' | '-'))
                    .is_some()
                {}

                let end = *self.inner.peek().map(|(i, _)| i).unwrap_or(&self.s.len());

                Ident(&self.s[start..end])
            }
            (_, c) => UnknownChar(c),
        };

        Some(tok)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        use crate::tokenizer::AbabaToken::*;
        let mut tok = AbabaTokenizer::new("{}[](),:   -123.456ababa");
        assert_eq!(tok.next(), Some(LeftBrace));
        assert_eq!(tok.next(), Some(RightBrace));
        assert_eq!(tok.next(), Some(LeftBracket));
        assert_eq!(tok.next(), Some(RightBracket));
        assert_eq!(tok.next(), Some(LeftParen));
        assert_eq!(tok.next(), Some(RightParen));
        assert_eq!(tok.next(), Some(ListSeparator));
        assert_eq!(tok.next(), Some(FieldSeparator));
        assert_eq!(tok.next(), Some(Number("-123.456")));
        assert_eq!(tok.next(), Some(Ident("ababa")));
        assert_eq!(tok.next(), None);
    }

    #[test]
    fn list() {
        use crate::tokenizer::AbabaToken::*;
        let mut tok = AbabaTokenizer::new("[1.0, 2, 3,4,5]");
        assert_eq!(tok.next(), Some(LeftBracket));
        assert_eq!(tok.next(), Some(Number("1.0")));
        assert_eq!(tok.next(), Some(ListSeparator));
        assert_eq!(tok.next(), Some(Number("2")));
        assert_eq!(tok.next(), Some(ListSeparator));
        assert_eq!(tok.next(), Some(Number("3")));
        assert_eq!(tok.next(), Some(ListSeparator));
        assert_eq!(tok.next(), Some(Number("4")));
        assert_eq!(tok.next(), Some(ListSeparator));
        assert_eq!(tok.next(), Some(Number("5")));
        assert_eq!(tok.next(), Some(RightBracket));
        assert_eq!(tok.next(), None);
    }

    #[test]
    fn idents() {
        use crate::tokenizer::AbabaToken::*;
        let mut tok = AbabaTokenizer::new("-aba-ba ababa _aba_ba");
        assert_eq!(tok.next(), Some(Number("-")));
        assert_eq!(tok.next(), Some(Ident("aba-ba")));
        assert_eq!(tok.next(), Some(Ident("ababa")));
        assert_eq!(tok.next(), Some(Ident("_aba_ba")));
        assert_eq!(tok.next(), None);
    }
}
