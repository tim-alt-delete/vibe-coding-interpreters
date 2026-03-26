use crate::lexer::token::{Token, TokenKind};
use super::ast::{Expr, Stmt};
use super::error::ParseError;

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt<'a>>, ParseError> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.statement()?);
        }
        Ok(statements)
    }

    fn statement(&mut self) -> Result<Stmt<'a>, ParseError> {
        if self.match_token(&[TokenKind::Print]) {
            let expr = self.expression()?;
            self.consume(TokenKind::Semicolon, "Expected ';' after value.")?;
            Ok(Stmt::Print(expr))

        }else if self.match_token(&[TokenKind::Var]) {
            let name = self.consume(TokenKind::Identifier(String::new()), "Expected variable name.")?.clone();
            let initializer = if self.match_token(&[TokenKind::Equal]) {
                Some(self.expression()?)
            } else {
                None
            };
            self.consume(TokenKind::Semicolon, "Expected ';' after variable declaration.")?;
            Ok(Stmt::Var { name, initializer })
        } else {
            let expr = self.expression()?;
            self.consume(TokenKind::Semicolon, "Expected ';' after expression.")?;
            Ok(Stmt::Expression(expr))
        }
    }

    fn expression(&mut self) -> Result<Expr<'a>, ParseError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr<'a>, ParseError> {
        let mut expr = self.comparison()?;

        while self.match_token(&[TokenKind::BangEqual, TokenKind::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr<'a>, ParseError> {
        let mut expr = self.term()?;

        while self.match_token(&[TokenKind::Greater, TokenKind::GreaterEqual, TokenKind::Less, TokenKind::LessEqual]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr<'a>, ParseError> {
        let mut expr = self.factor()?;

        while self.match_token(&[TokenKind::Plus, TokenKind::Minus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr<'a>, ParseError> {
        let mut expr = self.unary()?;

        while self.match_token(&[TokenKind::Star, TokenKind::Slash]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr<'a>, ParseError> {
        if self.match_token(&[TokenKind::Bang, TokenKind::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            })
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr<'a>, ParseError> {
        if self.match_token(&[TokenKind::False, TokenKind::True, TokenKind::Nil]) {
            return Ok(Expr::Literal(self.previous().clone()));
        }

        if self.match_token(&[TokenKind::NumberLiteral(String::new()), TokenKind::StringLiteral(String::new())]) {
            return Ok(Expr::Literal(self.previous().clone()));
        }

        if self.match_token(&[TokenKind::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenKind::RightParen, "Expected ')' after expression.")?;
            return Ok(Expr::Grouping(Box::new(expr)));
        }

        Err(ParseError::UnexpectedToken {
            expected: "expression".into(),
            found: format!("{:?}", self.peek().kind),
            span: self.peek().span,
        })
    }

    fn match_token(&mut self, kinds: &[TokenKind]) -> bool {
        for kind in kinds {
            if self.check(kind) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, kind: &TokenKind) -> bool {
        if self.is_at_end() {
            return false;
        }
        std::mem::discriminant(&self.peek().kind) == std::mem::discriminant(kind)
    }

    fn advance(&mut self) -> &Token<'a> {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek().kind, TokenKind::Eof)
    }

    fn peek(&self) -> &Token<'a> {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token<'a> {
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, kind: TokenKind, message: &str) -> Result<&Token<'a>, ParseError> {
        if self.check(&kind) {
            return Ok(self.advance());
        }
        Err(ParseError::UnexpectedToken {
            expected: format!("{:?}", kind),
            found: format!("{:?}", self.peek().kind),
            span: self.peek().span,
        })
    }

    // fn consume_identifier(&mut self, message: &str) -> Result<&Token<'a>, ParseError> {
    //     if let TokenKind::Identifier(_) = &self.peek().kind {
    //         return Ok(self.advance());
    //     }
    //     Err(ParseError::UnexpectedToken {
    //         expected: "identifier".into(),
    //         found: format!("{:?}", self.peek().kind),
    //         span: self.peek().span,
    //     })
    // }
}
