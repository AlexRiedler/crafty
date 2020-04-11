pub mod token;
use token::Token;
use token::TokenType;

use std::str::Chars;

use itertools::multipeek;
use itertools::structs::MultiPeek;

pub struct Scanner<'a> {
    src_iter: MultiPeek<Chars<'a>>,
    lexeme: String,
    line_number: u32,
}

impl Scanner<'_> {
    fn advance(&mut self) -> Option<char> {
        let ch = self.src_iter.next();

        if let Some(ch) = ch {
            self.lexeme.push(ch);
        }

        return ch;
    }

    fn scan_token(&mut self) -> Option<Token> {
        self.advance()
            .map(|ch| match ch {
                '(' => TokenType::LeftParen,
                ')' => TokenType::RightParen,
                '{' => TokenType::LeftBrace,
                '}' => TokenType::RightBrace,
                ',' => TokenType::Comma,
                '.' => TokenType::Dot,
                '-' => TokenType::Minus,
                '+' => TokenType::Plus,
                ';' => TokenType::Semicolon,
                '*' => TokenType::Star,
                '!' => match self.src_iter.peek() {
                    Some('=') => {
                        self.advance();
                        TokenType::BangEqual
                    }
                    _ => TokenType::Bang,
                },
                '=' => match self.src_iter.peek() {
                    Some('=') => {
                        self.advance();
                        TokenType::EqualEqual
                    }
                    _ => TokenType::Equal,
                },
                '<' => match self.src_iter.peek() {
                    Some('=') => {
                        self.advance();
                        TokenType::LessEqual
                    }
                    _ => TokenType::Equal,
                },
                '>' => match self.src_iter.peek() {
                    Some('=') => {
                        self.advance();
                        TokenType::GreaterEqual
                    }
                    _ => TokenType::Equal,
                },
                '/' => match self.src_iter.peek() {
                    Some('/') => {
                        while self.advance() != None {}
                        TokenType::Comment
                    }
                    _ => TokenType::Slash,
                },
                ' ' => TokenType::Whitespace,
                '\t' => TokenType::Whitespace,
                '\r' => TokenType::Whitespace,
                '\n' => {
                    self.line_number += 1;
                    TokenType::Whitespace
                }
                '"' => {
                    self.consume_string();
                    TokenType::Str
                }
                _ => if ch.is_digit(10) {
                    self.consume_number();
                    TokenType::Number
                } else if ch.is_alphabetic() {
                    self.consume_identifier();
                    self.identifier_token_type()
                } else {
                    TokenType::Unknown
                },
            })
            .map(|token_type| {
                let token = Token {
                    token_type: token_type,
                    lexeme: self.lexeme.clone(),
                    line_number: self.line_number,
                };
                self.lexeme = String::from("");
                token
            })
    }

    fn consume_string(&mut self) {
        while let Some(ch) = self.src_iter.peek() {
            if ch == &'"' {
                break;
            }
            self.advance();
        }
        self.advance();
    }

    fn consume_number(&mut self) {
        while let Some(ch) = self.src_iter.peek() {
            if !ch.is_digit(10) {
                break;
            }
            self.advance();
        }
        // TODO: handle decimal numbers
    }

    fn consume_identifier(&mut self) {
        while let Some(ch) = self.src_iter.peek() {
          if !ch.is_alphanumeric() {
              break;
          }
          self.advance();
        }
    }

    fn identifier_token_type(&mut self) -> TokenType {
        match self.lexeme.as_str() {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier
        }
    }
}

pub fn scan_tokens(source: &String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut scanner = Scanner {
        src_iter: multipeek(source.chars()),
        lexeme: String::from(""),
        line_number: 0u32,
    };

    while let Some(token) = scanner.scan_token() {
        tokens.push(token)
    }

    tokens.push(Token {
        token_type: TokenType::Eof,
        lexeme: String::from(""),
        line_number: scanner.line_number as u32,
    });
    tokens
}
