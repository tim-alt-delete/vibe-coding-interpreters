// hey, ./lexer.rs exists
pub mod lexer;
pub mod token;
pub mod error;

// Re-export commonly used types from submodules so callers can import them
// directly from `crate::lexer::{Lexer, Token, TokenKind, Span, LexerError}`
// instead of `crate::lexer::lexer::Lexer`, `crate::lexer::token::Token`, etc.
// (so you don't need to know the filename to import)
pub use lexer::Lexer;
pub use token::TokenKind;
//pub use token::{Token, TokenKind, Span};
//pub use error::LexerError;