use std::fmt::{Display, Formatter, Result};

/// Total possible types of Tokens, defined as TokenType. This is an enum over what Tokens can be.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    // General or literal
    NUMBER, // ex: 1234
    STRING, // "Hello World"

    // Single-character tokens
    PLUS, // +
    MINUS, // -
    LEFT_PAREN, // (
    RIGHT_PAREN, // )
    LEFT_BRACE, // {
    RIGHT_BRACE, // }
    LEFT_SQBR, // [
    RIGHT_SQBR, // ]
    COMMA, // ,
    DOT, // .
    SEMICOLON,
    SLASH, // the '/' character
    STAR, // *

    // Single or double, special types
    BANG, // !
    BANG_EQ, // !=
    EQ, // =
    EQ_EQ, // ==
    GREATER, // >
    GREATER_EQ, // >=
    LESS,
    LESS_EQ,

    // Keywords
    FOR,
    GLOBAL,
    LET,


    // Miscellaneous
    NEWLINE,
    EOF,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // Update this for every new token added
        match self {
            // General
            TokenType::NUMBER => write!(f, "NUMBER"),
            TokenType::STRING => write!(f, "STRING"),

            // Single-Characters
            TokenType::PLUS => write!(f, "PLUS"),
            TokenType::MINUS => write!(f, "MINUS"),
            TokenType::LEFT_PAREN => write!(f, "LEFT_PAREN"),
            TokenType::RIGHT_PAREN => write!(f, "RIGHT_PAREN"),
            TokenType::LEFT_BRACE => write!(f, "LEFT_BRACE"),
            TokenType::RIGHT_BRACE => write!(f, "RIGHT_BRACE"),
            TokenType::LEFT_SQBR => write!(f, "LEFT_SQBR"),
            TokenType::RIGHT_SQBR => write!(f, "RIGHT_SQBR"),
            TokenType::COMMA => write!(f, "COMMA"),
            TokenType::DOT => write!(f, "DOT"),
            TokenType::SEMICOLON => write!(f, "SEMICOLON"),
            TokenType::SLASH => write!(f, "SLASH"),
            TokenType::STAR => write!(f, "STAR"),

            // Special single or double
            TokenType::BANG => write!(f, "BANG"),
            TokenType::BANG_EQ => write!(f, "BANG_EQ"),
            TokenType::EQ => write!(f, "EQ"),
            TokenType::EQ_EQ => write!(f, "EQ_EQ"),
            TokenType::GREATER => write!(f, "GREATER"),
            TokenType::GREATER_EQ => write!(f, "GREATER_EQ"),
            TokenType::LESS => write!(f, "LESS"),
            TokenType::LESS_EQ => write!(f, "LESS_EQ"),

            // Keywords
            TokenType::FOR => write!(f, "FOR"),
            TokenType::GLOBAL => write!(f, "GLOBAL"),
            TokenType::LET => write!(f, "LET"),


            // Miscellaneous
            TokenType::NEWLINE => write!(f, "NEWLINE"),
            TokenType::EOF => write!(f, "EOF"),
        }
    }
}