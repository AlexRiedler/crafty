use crate::scanner::token::Token;
use crate::scanner::token::TokenType;

use std::iter::Peekable;
use core::slice::Iter;

pub struct ParseError {
    pub message: String,
}

pub struct Parser<'a> {
    pub iter: Peekable<Iter<'a, Token>>,
    pub current: Option<&'a Token>,
    pub previous: Option<&'a Token>,
}

pub enum Statement {
    Expression(Box<Expr>),
    Print(Box<Expr>),
    Var(Token, Option<Box<Expr>>),
}

pub enum Expr {
    Grouping(Box<Expr>),
    Binary(Box<Expr>, Box<Expr>, Box<Expr>),
    Unary(Box<Expr>, Box<Expr>),
    Operator(TokenType, String),
    BoolLiteral(bool),
    StringLiteral(String),
    IntegerLiteral(String),
    FloatLiteral(String),
    Variable(Token),
}

pub trait Visitor<T> {
    fn visit_expr(&mut self, e: &Expr) -> T;
    fn visit_statement(&mut self, s: &Statement) -> T;
}

impl Parser<'_> {
    pub fn parse(&mut self) -> Result<Vec<Statement>, ParseError> {
        self.advance();
        let mut statements: Vec<Statement> = Vec::new();

        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        return Ok(statements);
    }

    fn is_at_end(&mut self) -> bool {
        match self.current {
            Some(token) => token.token_type == TokenType::Eof,
            None => true
        }
    }

    fn advance(&mut self) -> Option<&'_ Token> {
        self.previous = self.current;
        self.current = self.iter.next();
        self.current
    }

    fn previous_token(&mut self) -> Result<Box<Expr>, ParseError> {
        return match &self.previous {
            Some(token) => Ok(Box::new(Expr::Operator(token.token_type.clone(), token.lexeme.to_string()))),
            None => Err(self.error("Internal Parser Error: No previous token found".to_string())),
        }
    }

    // DEFER: synchronizaton on ParseError (8.2.2)
    fn declaration(&mut self) -> Result<Statement, ParseError> {
        if self.token_match(&[TokenType::Var]) {
            return self.var_declaration();
        }

        self.statement()
    }

    fn var_declaration(&mut self) -> Result<Statement, ParseError> {
        let name = self.consume(TokenType::Identifier)?; // TODO: error message different

        let mut initializer = None;
        if self.token_match(&[TokenType::Equal]) {
            initializer = Some(self.expression()?);
        }

        self.consume(TokenType::Semicolon)?;
        Ok(Statement::Var(name, initializer))
    }

    fn statement(&mut self) -> Result<Statement, ParseError> {
        if self.token_match(&[TokenType::Print]) {
            return self.print_statement();
        }

        return self.expression_statement();
    }

    fn print_statement(&mut self) -> Result<Statement, ParseError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon)?;
        Ok(Statement::Print(value))
    }

    fn expression_statement(&mut self) -> Result<Statement, ParseError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon)?;
        Ok(Statement::Expression(value))
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

        while self.token_match(&[TokenType::Slash, TokenType::Star]) {
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
        if self.token_match(&[TokenType::Integer]) {
            match &self.previous {
                Some(token) => return Ok(Box::new(Expr::IntegerLiteral(token.lexeme.to_string()))),
                None => return Err(self.error("I DONT KNOW WHAT HAPPENED".to_string()))
            }
        }
        if self.token_match(&[TokenType::Float]) {
            match &self.previous {
                Some(token) => return Ok(Box::new(Expr::FloatLiteral(token.lexeme.to_string()))),
                None => return Err(self.error("I DONT KNOW WHAT HAPPENED".to_string()))
            }
        }
        if self.token_match(&[TokenType::Str]) {
            match &self.previous {
                Some(token) => return Ok(Box::new(Expr::StringLiteral(token.lexeme.to_string()))),
                None => return Err(self.error("I DONT KNOW WHAT HAPPENED".to_string()))
            }
        }
        if self.token_match(&[TokenType::Identifier]) {
            match &self.previous {
                Some(token) => return Ok(Box::new(Expr::Variable((**token).clone()))),
                None => return Err(self.error("I DONT KNOW WHAT HAPPENED".to_string()))
            }
        }

        if self.token_match(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen)?;
            return Ok(Box::new(Expr::Grouping(expr)));
        }

        Err(self.error("Expected literal".to_string()))
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

    fn consume(&mut self, token_type: TokenType) -> Result<Token, ParseError> {
        if self.check(&token_type) {
            let result =
                match self.current {
                    Some(token) => Ok(token.clone()),
                    None => Err(self.error(format!("advanced past end on token check"))) // should be unreachable
                };
            self.advance();
            result
        } else {
            Err(self.error(format!("expected {:?} after expression", token_type)))
        }
    }

    fn check(&mut self, token_type: &TokenType) -> bool {
        match self.current {
            Some(token) => &token.token_type == token_type,
            None => false
        }
    }

    fn error(&mut self, message: String) -> ParseError {
        match self.current {
            Some(token) =>
                match token.token_type {
                    TokenType::Eof => ParseError{message: format!("{} at end of file {}:{}", message, token.line_number, token.column_number) },
                    _ => ParseError{message: format!("{} at '{}' line {}:{}", message, token.lexeme, token.line_number, token.column_number) },
                }
            None => ParseError{message: format!("unexpected EOF: {}", message)}
        }
    }
}
