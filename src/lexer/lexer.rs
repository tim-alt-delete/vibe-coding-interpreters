use std::iter::Peekable;
use std::str::Chars;

use crate::lexer::token::{Token, TokenKind, Span};
use crate::lexer::error::LexerError;

pub struct Lexer<'a> {
    input: &'a str,
    chars: Peekable<Chars<'a>>,
    current_pos: usize, // byte offset
    start_pos: usize,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input,
            chars: input.chars().peekable(),
            current_pos: 0,
            start_pos: 0,
            line: 1,
            column: 1,
        }
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.chars.next()?;
        self.current_pos += c.len_utf8();
        if c == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(c)
    }

    // calling peek requires mutable borrow because 
    // it may advance the iterator internally
    fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn make_token(&self, kind: TokenKind) -> Result<Token<'a>, LexerError> {
        Ok(Token {
            kind,
            lexeme: &self.input[self.start_pos..self.current_pos],
            span: self.current_span(),
        })
    }

    fn current_span(&self) -> Span {
        Span {
            start: self.start_pos,
            end: self.current_pos,
            line: self.line,
            column: self.column,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            match c {
                ' ' | '\t' | '\r' | '\n' => { self.advance(); }
                _ => break,
            }
        }
    }

    pub fn next_token(&mut self) -> Result<Token<'a>, LexerError> {
        self.skip_whitespace();
        self.start_pos = self.current_pos;

        let Some(c) = self.advance() else {
            return self.make_token(TokenKind::Eof);
        };

        match c {
            '(' => self.make_token(TokenKind::LeftParen),
            ')' => self.make_token(TokenKind::RightParen),
            '{' => self.make_token(TokenKind::LeftBrace),
            '}' => self.make_token(TokenKind::RightBrace),
            '[' => self.make_token(TokenKind::LeftBracket),
            ']' => self.make_token(TokenKind::RightBracket),
            ',' => self.make_token(TokenKind::Comma),
            '.' => self.make_token(TokenKind::Dot),
            '-' => self.make_token(TokenKind::Minus),
            '+' => self.make_token(TokenKind::Plus),
            ';' => self.make_token(TokenKind::Semicolon),
            '*' => self.make_token(TokenKind::Star),
            '/' => self.make_token(TokenKind::Slash),

            '!' => {
                if self.match_next('=') {
                    self.make_token(TokenKind::BangEqual)
                } else {
                    self.make_token(TokenKind::Bang)
                }
            }

            '=' => {
                if self.match_next('=') {
                    self.make_token(TokenKind::EqualEqual)
                } else {
                    self.make_token(TokenKind::Equal)
                }
            }

            '<' => {
                if self.match_next('=') {
                    self.make_token(TokenKind::LessEqual)
                } else {
                    self.make_token(TokenKind::Less)
                }
            }

            '>' => {
                if self.match_next('=') {
                    self.make_token(TokenKind::GreaterEqual)
                } else {
                    self.make_token(TokenKind::Greater)
                }
            }

            '"' => {
                while let Some(ch) = self.peek() {
                    if ch == '"' {
                        self.advance();
                        let slice = &self.input[self.start_pos + 1..self.current_pos - 1];
                        return self.make_token(TokenKind::StringLiteral(slice.to_string()));
                    }
                    self.advance();
                }
                Err(LexerError::UnterminatedString {
                    span: self.current_span(),
                })
            }

            c if c.is_ascii_digit() => {
                while let Some(ch) = self.peek() {
                    if ch.is_ascii_digit() || ch == '.' {
                        self.advance();
                    } else {
                        break;
                    }
                }
                let slice = &self.input[self.start_pos..self.current_pos];
                self.make_token(TokenKind::NumberLiteral(slice.to_string()))
            }

            c if c.is_ascii_alphabetic() || c == '_' => {
                while let Some(ch) = self.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        self.advance();
                    } else {
                        break;
                    }
                }
                let ident = &self.input[self.start_pos..self.current_pos];

                let kind = match ident {
                    "and"    => TokenKind::And,
                    "class"  => TokenKind::Class,
                    "else"   => TokenKind::Else,
                    "false"  => TokenKind::False,
                    "fun"    => TokenKind::Fun,
                    "for"    => TokenKind::For,
                    "if"     => TokenKind::If,
                    "nil"    => TokenKind::Nil,
                    "or"     => TokenKind::Or,
                    "print"  => TokenKind::Print,
                    "return" => TokenKind::Return,
                    "super"  => TokenKind::Super,
                    "this"   => TokenKind::This,
                    "true"   => TokenKind::True,
                    "var"    => TokenKind::Var,
                    "while"  => TokenKind::While,
                    "echo"   => TokenKind::Echo,
                    "cd"     => TokenKind::Cd,
                    "exit"   => TokenKind::Exit,
                    _        => TokenKind::Identifier(ident.to_string()),
                };

                self.make_token(kind)
            }

            _ => Err(LexerError::UnexpectedChar {
                char: c,
                span: self.current_span(),
            }),
        }
    }
}

// Example Usage
// let mut lexer = Lexer::new("if (x == 10) print x;");
// while let token = lexer.next_token() {
//     println!("{:?}", token);
//     if matches!(token.kind, TokenKind::Eof) { break; }
// }


// Unit tests for the lexer
// $ cargo test
#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::token::TokenKind;

    fn lex_all(source: &str) -> Vec<Token> {
        let mut lexer = Lexer::new(source);
        let mut tokens = Vec::new();
        while let Ok(token) = lexer.next_token() {
            if matches!(token.kind, TokenKind::Eof) {
                break;
            }
            tokens.push(token);
        }
        tokens
    }

    #[test]
    fn test_single_char_tokens() {
        let source = "(){},.-+;*/";
        let kinds = vec![
            TokenKind::LeftParen,
            TokenKind::RightParen,
            TokenKind::LeftBrace,
            TokenKind::RightBrace,
            TokenKind::Comma,
            TokenKind::Dot,
            TokenKind::Minus,
            TokenKind::Plus,
            TokenKind::Semicolon,
            TokenKind::Star,
            TokenKind::Slash,
        ];

        let tokens = lex_all(source);
        let actual_kinds: Vec<_> = tokens.into_iter().map(|t| t.kind).collect();
        assert_eq!(actual_kinds, kinds);
    }

    #[test]
    fn test_double_char_tokens() {
        let source = "!= == <= >=";
        let kinds = vec![
            TokenKind::BangEqual,
            TokenKind::EqualEqual,
            TokenKind::LessEqual,
            TokenKind::GreaterEqual,
        ];

        let tokens = lex_all(source);
        let actual_kinds: Vec<_> = tokens.into_iter().map(|t| t.kind).collect();
        assert_eq!(actual_kinds, kinds);
    }

    #[test]
    fn test_identifiers_and_keywords() {
        let source = "var echo exit print";
        let kinds = vec![
            TokenKind::Var,
            TokenKind::Echo,
            TokenKind::Exit,
            TokenKind::Print,
        ];

        let tokens = lex_all(source);
        let actual_kinds: Vec<_> = tokens.into_iter().map(|t| t.kind).collect();
        assert_eq!(actual_kinds, kinds);
    }

    #[test]
    fn test_numbers_and_strings() {
        let source = "42 3.14 \"hello\"";
        let tokens = lex_all(source);
        assert!(matches!(tokens[0].kind, TokenKind::NumberLiteral(ref n) if n == "42"));
        assert!(matches!(tokens[1].kind, TokenKind::NumberLiteral(ref n) if n == "3.14"));
        assert!(matches!(tokens[2].kind, TokenKind::StringLiteral(ref s) if s == "hello"));
    }

    #[test]
    fn test_unterminated_string_error() {
        let mut lexer = Lexer::new("\"unterminated");
        let result = lexer.next_token();
        assert!(matches!(result, Err(LexerError::UnterminatedString { .. })));
    }

    #[test]
    fn test_unexpected_char_error() {
        let mut lexer = Lexer::new("@");
        let result = lexer.next_token();
        assert!(matches!(result, Err(LexerError::UnexpectedChar { char: '@', .. })));
    }
}
