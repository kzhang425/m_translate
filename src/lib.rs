#![allow(non_camel_case_types)]

// Major implementation of behind-the-scenes stuff

use std::fmt::{Display, Formatter, Result};
use std::fs::File;
use std::io;
use std::io::{Read};

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







//----------TOKEN DEFINITION AND IMPLEMENTATION----------
#[derive(PartialEq, Debug)]
pub struct Token {
    ttype: TokenType,
    lexeme: String,
    literal: String, // Special case, since in the Java implementation it is simply Object (very general)
    line: usize,
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: String, literal: String, line: usize) -> Self {
        Token {
            ttype,
            lexeme,
            literal,
            line,
        }
    }
    pub fn repr(&self) -> String {
        let t = self.ttype;
        if self.literal.len() == 0 {
            return format!("Token({}, {}, None, {})", t.to_string(), self.lexeme, self.line);
        } else {
            return format!("Token({}, {}, {}, {})", t.to_string(), self.lexeme, self.literal, self.line);
        } 
    }

    pub fn get_literal(&self) -> String {
        self.literal.clone()
    }
}










// ----------SCANNER----------
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,

    // Keeping track of where the scanner is in the file
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    // Takes a file and consumes it, passing it into the scanner
    pub fn from_file(mut f: File) -> io::Result<Self> {
        let mut buf = String::new();
        f.read_to_string(&mut buf)?; // the method returns the number of bytes read to the buffer
        Ok(Self {
            source: buf,
            tokens: Vec::<Token>::new(),

            start: 0,
            current: 0,
            line: 1,
        })
    }

    pub fn from_string(s: String) -> Self {
        Self {
            source: s,
            tokens: Vec::<Token>::new(),

            start: 0,
            current: 0,
            line: 1,
        }
    }

    // A hardly used function, only used to verify that the struct Scanner does indeed get created
    pub fn get_source(&self) -> &str {
        let s = &self.source;
        s
    }

    pub fn into_tokens(self) -> Vec<Token> {
        self.tokens
    }








    //----------TOKEN LOGIC----------


    // Call this method to process the text within the Scanner
    pub fn scan_tokens(&mut self) {
        let mut total_iterations = 0;

        // If this loop somehow never finds an end, make sure to stop it at some point!
        while !self.is_at_end() && total_iterations < i32::MAX {
            self.start = self.current;
            self.scan_token();
            total_iterations += 1;
        }

        // Add the EOF Token to be safe and to be in line with other interpreters
        self.tokens.push(Token::new(TokenType::EOF, String::from(""), String::from(""), self.line));
    }

    #[inline]
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    // This is the true heart of the scanner, where most of the logic should be implemented
    fn scan_token(&mut self) {
        let c = self.advance().unwrap();
        // c contains a char but the "current pointer" has now been shifted to the next position

        // This accounts for most of the basic tokens and other stuff
        match c {
            '(' => self.add_token_no_literal(TokenType::LEFT_PAREN),
            ')' => self.add_token_no_literal(TokenType::RIGHT_PAREN),
            '{' => self.add_token_no_literal(TokenType::LEFT_BRACE),
            '}' => self.add_token_no_literal(TokenType::RIGHT_BRACE),
            '[' => self.add_token_no_literal(TokenType::LEFT_SQBR),
            ']' => self.add_token_no_literal(TokenType::RIGHT_SQBR),
            ',' => self.add_token_no_literal(TokenType::COMMA),
            '.' => self.add_token_no_literal(TokenType::DOT),
            '-' => self.add_token_no_literal(TokenType::MINUS),
            '+' => self.add_token_no_literal(TokenType::PLUS),
            ';' => self.add_token_no_literal(TokenType::SEMICOLON),
            '*' => self.add_token_no_literal(TokenType::STAR),

            // Now we have characters that have potential double meanings. The bang operator is the best example
            '!' => self.add_token_conditional('=', TokenType::BANG_EQ, TokenType::BANG),
            '=' => self.add_token_conditional('=', TokenType::EQ_EQ, TokenType::EQ),
            '<' => self.add_token_conditional('=', TokenType::LESS_EQ, TokenType::LESS),
            '>' => self.add_token_conditional('=', TokenType::GREATER_EQ, TokenType::GREATER),


            // Characters that dont do anything or are simply delimiters
            ' ' | '\r' | '\t' => (),


            // Newline handled here. Increment by 1 and add its associated token
            '\n' => {
                self.line += 1;
                
                // Keep track of newlines as the generated MUMPS code needs to have organization too
                self.add_token_no_literal(TokenType::NEWLINE);
            }


            // The slash character can be used in comments and division, so it should be handled with two cases
            '/' => {
                if self.conditional_advance('/') {
                    // Then this is a comment and goes until the end of the line. Ignore this for compilation.
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    // Otherwise, this is a token worth looking at
                    self.add_token_no_literal(TokenType::SLASH);
                }
            }


            // For string literals, things get a little more complicated
            '"' => {
                while self.peek() != '"' && !self.is_at_end() {
                    // peeks through the duration of the string, 
                    if self.peek() == '\n' {
                        self.line += 1;
                    }

                    // lets probably implement the escape sequence of \" so that we can have quotes in strings
                    if self.peek() == '\\' {
                        // just simply skip the character and therefore don't apply special rules to the character after the backslash
                        self.advance();
                    }

                    self.advance();
                }

                // If we don't see another quote for a while then maybe we have an unclosed quote
                if self.is_at_end() {
                    panic!("Unclosed quote for string literal.");
                }

                // Skip past the closing quote and now the current index is one after it
                self.advance();

                // Collect the characters between the two indexes we have: start and current
                let start_no_quote = self.start + 1;
                let current_no_quote = self.current - 1;
                let resultant_string = String::from(&self.source[start_no_quote..current_no_quote]);

                // Now this string still contains escape sequences but we'll keep them in as this is the literal string
                // Make sure to process them during the codegen step by expanding the token and processing the literal
                // contained within.
                self.add_token(TokenType::STRING, resultant_string);
                
            }


            // Default case analogous to default: in a switch statement in other languages
            _ => {
                println!("Unexpected character found in line {}", self.line);
            }
        }



    }







    // Helper functions, generally not intended for direct use API-wise
    // Some of them are inlined as to reduce overhead. Because these are so small, inlining
    // could potentially be helpful for performance even if it is marginal. Also, it's not 
    // hard to do: just attach an attribute above said function.

    fn add_token(&mut self, ttype: TokenType, literal: String) {
        let text = String::from(&self.source[self.start..self.current]);
        self.tokens.push(Token::new(ttype, text, literal, self.line));

    }

    #[inline]
    fn add_token_no_literal(&mut self, ttype: TokenType) {
        self.add_token(ttype, String::from(""));
    }

    fn add_token_conditional(&mut self, expected_char: char, expected_type: TokenType, base_type: TokenType) {
        if self.conditional_advance(expected_char) {
            self.add_token_no_literal(expected_type);
        } else {
            self.add_token_no_literal(base_type);
        }
    }

    // How the advance function works is that it first grabs the value at the current then increments current
    // This means that after each advance() call, the current is actually unread thus far
    #[inline]
    fn advance(&mut self) -> Option<char> {
        let c = self.source.chars().nth(self.current);
        self.current += 1;

        c
    }

    // holds an immutable reference to self, can't mutate data, only peeks
    #[inline]
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        } else {
            self.source.chars().nth(self.current).unwrap()
        }
    }

    // If we have a unit like "!=", make sure to fully capture this as its own token rather than two separate things
    // Also, the online tutorial calls this function "match" but match is a keyword here in Rust.
    #[inline]
    fn conditional_advance(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        } else if expected != self.source.chars().nth(self.current).unwrap() {
            return false;
        } else {
            self.current += 1;
            true
        }
    }
}







//----------CONVENIENCE FUNCTIONS----------

#[inline]
fn empty_string() -> String {
    String::from("")
}

/// Takes a usize describing what line to designate the end of file at.
#[inline]
fn eof_token(lin: usize) -> Token {
    Token::new(TokenType::EOF, empty_string(), empty_string(), lin)
}

// To easily make scanners for testing, here are some more functions to create things
// These are just one-call functions that make a scanner with a predetermined buffer
fn helloworld_scanner() -> Scanner {
    Scanner::from_string("Hello World".to_string())
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn use_stringify() {
        // When you stringify an enum type, it looks like this:
        let a = stringify!(TokenType::INTEGER);
        assert_eq!(a, String::from("TokenType :: INTEGER"));
    }

    #[test]
    fn step_thru_chars() {
        let s = String::from("Hello World");
        assert_eq!('H', s.chars().nth(0).unwrap());
    }

    #[test]
    fn make_scanner() {
        let s = String::from("Hello World");
        let scanner = Scanner::from_string(s.clone());

        assert_eq!(s, String::from(scanner.get_source()));
    }

    #[test]
    fn test_advance() {
        let s = String::from("Hello World");
        let mut scanner = Scanner::from_string(s.clone());

        for i in 0..s.len() {
            let c = scanner.advance().unwrap();
            assert_eq!(c, s.chars().nth(i).unwrap());
        }
    }

    #[test]
    fn basic_token_gen() {
        let s = String::from("()[]");
        let mut scanner = Scanner::from_string(s);

        // Can't use s beyond here

        scanner.scan_tokens();
        let compare = vec![Token::new(TokenType::LEFT_PAREN, String::from("("), empty_string(), 1),
            Token::new(TokenType::RIGHT_PAREN, String::from(")"), empty_string(), 1),
            Token::new(TokenType::LEFT_SQBR, String::from("["), empty_string(), 1),
            Token::new(TokenType::RIGHT_SQBR, String::from("]"), empty_string(), 1),
            Token::new(TokenType::EOF, empty_string(), empty_string(), 1)];

        assert_eq!(scanner.into_tokens(), compare);
    }

    #[test]
    fn string_parse() {
        let s = String::from("\"This is a string\"");
        let mut scanner = Scanner::from_string(s);

        // Scan this to see if it will detect it as a string literal
        scanner.scan_tokens();
        let tokens = scanner.into_tokens();
        let first = tokens[0].get_literal();

        assert_eq!(first, "This is a string".to_string());
    }


}