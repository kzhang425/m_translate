#![allow(non_camel_case_types)]
pub mod tokens;
pub mod scan;

// Collect all modules here
use tokens::{ttypes::*, token::*};
use scan::{scanner::*};



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