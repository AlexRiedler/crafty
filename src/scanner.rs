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
    UnexpectedEol(),
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

    fn scan_token(&mut self) -> Result<Token, Error> {
        let token_type_result = match self.advance() {
            Some('(') => Ok(TokenType::LeftParen),
            Some(')') => Ok(TokenType::RightParen),
            Some('{') => Ok(TokenType::LeftBrace),
            Some('}') => Ok(TokenType::RightBrace),
            Some(',') => Ok(TokenType::Comma),
            Some('.') => Ok(TokenType::Dot),
            Some('-') => Ok(TokenType::Minus),
            Some('+') => Ok(TokenType::Plus),
            Some(';') => Ok(TokenType::Semicolon),
            Some('*') => Ok(TokenType::Star),
            Some('!') => match self.src_iter.peek() {
                Some('=') => {
                    self.advance();
                    Ok(TokenType::BangEqual)
                }
                _ => Ok(TokenType::Bang),
            },
            Some('=') => match self.src_iter.peek() {
                Some('=') => {
                    self.advance();
                    Ok(TokenType::EqualEqual)
                }
                _ => Ok(TokenType::Equal),
            },
            Some('<') => match self.src_iter.peek() {
                Some('=') => {
                    self.advance();
                    Ok(TokenType::LessEqual)
                }
                _ => Ok(TokenType::Equal),
            },
            Some('>') => match self.src_iter.peek() {
                Some('=') => {
                    self.advance();
                    Ok(TokenType::GreaterEqual)
                }
                _ => Ok(TokenType::Equal),
            },
            Some('/') => match self.src_iter.peek() {
                Some('/') => {
                    while self.advance() != None {}
                    Ok(TokenType::Comment)
                }
                _ => Ok(TokenType::Slash),
            },
            Some(ch) => {
                return Err(Error::Lexical(
                    self.line_number,
                    self.lexeme.clone(),
                    format!("Unexpected character '{}'", ch).to_string(),
                ))
            }
            None => return Err(Error::UnexpectedEol()),
        };

        token_type_result.map(|token_type| Token {
            token_type: token_type,
            lexeme: self.lexeme.clone(),
            line_number: self.line_number,
            char_index: self.index,
        })
    }
}

pub fn scan_tokens(source: &String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let lines: Vec<&str> = source.split("\n").collect();
    for (line_no, line) in lines.iter().enumerate() {
        let mut scanner = Scanner {
            src_iter: multipeek(line.chars()),
            lexeme: String::from(""),
            line_number: line_no as u32,
            index: 0u32,
        };

        match scanner.scan_token() {
            Ok(token) => tokens.push(token),
            Err(_e) => {}
        }
    }

    tokens.push(Token {
        token_type: TokenType::Eof,
        lexeme: String::from(""),
        line_number: lines.len() as u32,
        char_index: 0 as u32,
    });
    tokens
}
