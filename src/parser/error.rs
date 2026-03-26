use crate::lexer::token::Span;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken {
        expected: String,
        found: String,
        span: Span,
    },
    UnexpectedEOF {
        expected: String,
        span: Span,
    },
}

impl std::error::Error for ParseError {}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedToken { expected, found, span } =>
                write!(f, "Expected {}, but found {} at {}:{} ({}..{})", expected, found, span.line, span.column, span.start, span.end),
            ParseError::UnexpectedEOF { expected, span } =>
                write!(f, "Expected {}, but found end of file at {}:{}", expected, span.line, span.column),
        }
    }
}