use super::token::Span;
use std::fmt;

#[derive(Debug)]
pub enum LexerError {
    UnexpectedChar {
        char: char,
        span: Span,
    },
    UnterminatedString {
        span: Span,
    },
    InvalidNumber {
        lexeme: String,
        span: Span,
    },
    // Add more as needed
}

impl std::error::Error for LexerError {}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerError::UnexpectedChar { char, span } =>
                write!(f, "Unexpected '{}' at {}:{} ({}..{})", char, span.line, span.column, span.start, span.end),
            LexerError::UnterminatedString { span } =>
                write!(f, "Unterminated string at {}:{}", span.line, span.column),
            LexerError::InvalidNumber { lexeme, span } =>
                write!(f, "Invalid number `{}` at {}:{}", lexeme, span.line, span.column),
        }
    }
}
