
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single-character tokens.                      
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.                  
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.                                     
    Identifier,
    Str,
    Integer,
    Float,
    Comment,

    // Keywords.                                     
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Whitespace,
    Newline,
    Unknown,
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line_number: u32,
    pub column_number: u32,
}
