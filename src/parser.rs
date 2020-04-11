use crate::scanner::token::Token;
use crate::scanner::token::TokenType;
use itertools::structs::MultiPeek;
use core::slice::Iter;

pub struct ParseError;

pub struct Parser<'a> {
    iter: MultiPeek<Iter<'a, Token>>,
    current: Option<'a Token>,
    previous: Option<'a Token>,
}

pub enum Expr {
    Binary(Box<Expr>, Box<Token>, Box<Expr>),
    Unary(Box<Token>, Box<Expr>),
    BoolLiteral(bool),
}

impl Parser<'_> {
    fn advance(&mut self) -> Option<&'_ Token> {
        self.previous = self.current;
        self.current = self.iter.next();
        self.current
    }

    fn previous_token(&mut self) -> Result<Box<Token>, ParseError> {
        return match self.previous {
            Some(token) => Ok(Box::new(*token)),
            None => Err(ParseError{}),
        }
    }

    fn expression(&mut self) -> Result<Box<Expr>, ParseError> {
        return self.equality();
    }

    fn equality(&mut self) -> Result<Box<Expr>, ParseError> {
        let mut expr = self.comparison()?;

        while self.token_match(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous_token()?;
            let right = self.comparison()?;
            expr = Box::new(Expr::Binary(expr, operator, right));
        }

        return Ok(expr);
    }

    fn comparison(&mut self) -> Result<Box<Expr>, ParseError> {
        let mut expr = self.addition()?;

        while self.token_match(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = self.previous_token()?;
            let right = self.addition()?;
            expr = Box::new(Expr::Binary(expr, operator, right));
        }

        return Ok(expr);
    }

    fn addition(&mut self) -> Result<Box<Expr>, ParseError> {
        let mut expr = self.multiplication()?;

        while self.token_match(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous_token()?;
            let right = self.multiplication()?;
            expr = Box::new(Expr::Binary(expr, operator, right));
        }

        return Ok(expr);
    }

    fn multiplication(&mut self) -> Result<Box<Expr>, ParseError> {
        let mut expr = self.unary()?;

        while self.token_match(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous_token()?;
            let right = self.unary()?;
            expr = Box::new(Expr::Binary(expr, operator, right));
        }

        return Ok(expr);
    }

    fn unary(&mut self) -> Result<Box<Expr>, ParseError> {
        if self.token_match(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous_token()?;
            let right = self.unary()?;
            return Ok(Box::new(Expr::Unary(operator, right)));
        }

        return self.primary();
    }

    fn primary(&mut self) -> Result<Box<Expr>, ParseError> {
        if self.token_match(&[TokenType::False]) {
            return Ok(Box::new(Expr::BoolLiteral(false)));
        }
        if self.token_match(&[TokenType::True]) {
            return Ok(Box::new(Expr::BoolLiteral(true)));
        }
        if self.token_match(&[TokenType::Number]) {
            // return Ok(Box::new(Expr::NumberLiteral(self.previous.lexeme)));
        }

        return Err(ParseError{})
    }

    fn token_match(&mut self, token_types: &[TokenType]) -> bool {
        match self.current {
            Some(token) => {
                for token_type in token_types {
                    if token.token_type == *token_type {
                        self.advance();
                        return true;
                    }
                }
                return false
            }
            None => return false
        }
    }
}
