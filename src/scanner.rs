pub mod token;
use token::Token;
use token::TokenType;

use std::str::Chars;

use itertools::multipeek;
use itertools::structs::MultiPeek;

#[derive(Debug, PartialEq)]
pub enum Error {
    Lexical(u32, String, String),

    Runtime(String),
}

pub struct Scanner<'a> {
    src_iter: MultiPeek<Chars<'a>>,
    lexeme: String,
    line_number: u32,
    index: u32,
}

impl Scanner<'_> {
    fn advance(&mut self) -> Option<char> {
        let ch = self.src_iter.next();

        if let Some(ch) = ch {
            self.index += 1;
            self.lexeme.push(ch);
        }

        return ch;
    }

    fn advance_until(&mut self, until: char) -> Option<char> {
        let mut last = None;
        while let Some(ch) = self.src_iter.peek() {
            if *ch == until {
                break;
            } else {
                last = Some(*ch);
                self.advance();
            }
        }
        last
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
                _ => TokenType::Unknown,
            })
            .map(|token_type| {
                let token = Token {
                    token_type: token_type,
                    lexeme: self.lexeme.clone(),
                    line_number: self.line_number,
                    char_index: self.index,
                };
                self.lexeme = String::from("");
                token
            })
    }

    fn consume_string(&mut self) -> Result<String, Error> {
        while let Some(ch) = self.src_iter.peek() {
            if ch == &'"' {
                break;
            }
            self.advance();
        }

        match self.advance() {
            Some(_ch) => Ok(self.lexeme.clone()),
            None => Err(Error::Lexical(
                self.index,
                "Expected end of string".to_string(),
                "Did not find closing \"".to_string(),
            )),
        }
    }
}

pub fn scan_tokens(source: &String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut scanner = Scanner {
        src_iter: multipeek(source.chars()),
        lexeme: String::from(""),
        line_number: 0u32,
        index: 0u32,
    };

    while let Some(token) = scanner.scan_token() {
        tokens.push(token)
    }

    tokens.push(Token {
        token_type: TokenType::Eof,
        lexeme: String::from(""),
        line_number: scanner.line_number as u32,
        char_index: scanner.index + 1 as u32,
    });
    tokens
}
