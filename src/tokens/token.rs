use super::ttypes::*;

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