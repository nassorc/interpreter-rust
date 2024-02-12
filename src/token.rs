use std::collections::HashMap;

pub type TokenType = &'static str;
pub const ILLEGAL: &str = "ILLEGAL";
pub const EOF: &str = "EOF";
pub const IDENTIFIER: &str = "IDENTIFIER";
pub const LET: &str = "LET";
pub const INT: &str = "INT";
pub const FUNCTION: &str = "FN";
pub const RETURN: &str = "RETURN";
pub const IF: &str = "IF";
pub const ELSE: &str = "ELSE";
pub const TRUE: &str = "TRUE";
pub const FALSE: &str = "FALSE";

pub const COMMA: &str = ",";
pub const SEMICOLON: &str = ";";
pub const LPAREN: &str = "(";
pub const RPAREN: &str = ")";
pub const LBRACE: &str = "{";
pub const RBRACE: &str = "}";
pub const ASSIGN: &str = "=";
pub const PLUS: &str = "+";
pub const MINUS: &str = "-";
pub const ASTERISK: &str = "*";
pub const SLASH: &str = "/";
pub const BANG: &str = "!";
pub const LT: &str = "<";
pub const GT: &str = ">";
pub const EQ: &str = "==";
pub const NOTEQ: &str = "!=";



#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
  pub fn new(token_type: TokenType, literal: &str) -> Self {
    Self {
      token_type,
      literal: literal.to_string()
    }
  }
}

pub fn get_identifier(ident: &str) -> TokenType {
  let keyword_map: HashMap<&'static str, TokenType> = HashMap::from([
    ("let", LET),
    ("fn", FUNCTION),
    ("return", RETURN),
    ("if", IF),
    ("else", ELSE),
    ("true", TRUE),
    ("false", FALSE),
  ]);

  if keyword_map.contains_key(ident) {
    return keyword_map.get(ident).unwrap();
  }

  return IDENTIFIER;
}
