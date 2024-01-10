use std::collections::HashMap;

pub const ILLEGAL: &str = "ILLEGAL";
pub const EOF: &str = "EOF";

pub const IDENTIFIER: &str = "IDENTIFIER";
pub const INT: &str = "INT";

pub const COMMA: &str = ",";
pub const SEMICOLON: &str = ";";

pub const LPAREN: &str = "(";
pub const RPAREN: &str = ")";
pub const LBRACE: &str = "{";
pub const RBRACE: &str = "}";

// OPERATORS ///////////////
const PLUS: &str = "+";
const ASSIGN: &str = "=";
////////////////////////////
// KEYWORDS ////////////////
const LET: &str = "let";
const FUNCTION: &str = "fn";
////////////////////////////

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenType {
    ILLEGAL,
    EOF,
    IDENTIFIER,
    INT,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    PLUS,
    ASSIGN,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,
    EQ,
    NOTEQ,
    LET,
    FUNCTION,
    IF,
    ELSE,
    RETURN,
    TRUE,
    FALSE
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}
pub fn get_identifier(ident: &str) -> TokenType {
  let mut keyword_map: HashMap<String, TokenType> = HashMap::new();
  keyword_map.insert("let".to_string(), TokenType::LET);
  keyword_map.insert("fn".to_string(), TokenType::FUNCTION);
  keyword_map.insert("if".to_string(), TokenType::IF);
  keyword_map.insert("else".to_string(), TokenType::ELSE);
  keyword_map.insert("return".to_string(), TokenType::RETURN);
  keyword_map.insert("true".to_string(), TokenType::TRUE);
  keyword_map.insert("false".to_string(), TokenType::FALSE);

  // if ident in map, return value type
  if keyword_map.contains_key(ident) {
    return keyword_map[ident].clone();
  }
  // else return identifier type

  TokenType::IDENTIFIER
}