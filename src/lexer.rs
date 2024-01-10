use std::char::from_u32;
use crate::token::{Token, TokenType, get_identifier};

#[derive(Default, Debug)]
pub struct Lexer {
    pub input: String,
    pub position: usize,
    pub read_position: usize,
    pub ch: u8
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut l = Self {
            input: input.to_string(),
            ..Default::default()
        };
        l.read_char();
        return l;
    }

    pub fn next_token(&mut self) -> Token {
        let mut tok: Token;

        self.skip_whitespaces();

        let ch = String::from_utf8(vec![self.ch]).unwrap();


        match self.ch as char {
            '=' => { 
                // check EQUALITY
                if self.peek_char() == '=' {
                  let mut lpart = ch;
                  self.read_char();
                  let mut rpart = String::from_utf8(vec![self.ch]).unwrap();

                  lpart.push(self.ch as char);

                  tok = Self::new_token(TokenType::EQ, &lpart);
                } else {
                  tok = Self::new_token(TokenType::ASSIGN, &ch);
                }
            },
            '!' => { 
                // check EQUALITY
                if self.peek_char() == '=' {
                  let mut lpart = ch;
                  self.read_char();
                  let mut rpart = String::from_utf8(vec![self.ch]).unwrap();

                  lpart.push(self.ch as char);

                  tok = Self::new_token(TokenType::NOTEQ, &lpart);
                } else {
                  tok = Self::new_token(TokenType::BANG, &ch);
                }
            },
            ';' => { 
                tok = Self::new_token(TokenType::SEMICOLON, &ch);
            },
            '(' => { 
                tok = Self::new_token(TokenType::LPAREN, &ch);
            },
            ')' => { 
                tok = Self::new_token(TokenType::RPAREN, &ch);
            },
            ',' => { 
                tok = Self::new_token(TokenType::COMMA, &ch);
            },
            '+' => { 
                tok = Self::new_token(TokenType::PLUS, &ch);
            },
            '-' => { 
                tok = Self::new_token(TokenType::MINUS, &ch);
            },
            '*' => { 
                tok = Self::new_token(TokenType::ASTERISK, &ch);
            },
            '/' => { 
                tok = Self::new_token(TokenType::SLASH, &ch);
            },
            '<' => { 
                tok = Self::new_token(TokenType::LT, &ch);
            },
            '>' => { 
                tok = Self::new_token(TokenType::GT, &ch);
            },
            '{' => { 
                tok = Self::new_token(TokenType::LBRACE, &ch);
            },
            '}' => { 
                tok = Self::new_token(TokenType::RBRACE, &ch);
            },
            '\0' => {
                tok = Self::new_token(TokenType::EOF, "");
            }
            _ => {
                if Self::is_letter(self.ch as char) {
                    // let pos = self.position;
                    let literal = self.read_identifier();
                    let token_type = get_identifier(literal);
                    tok = Self::new_token(token_type, &literal);
                    return tok;
                } else if self.is_digit() {
                  let literal = self.read_digit();
                  return Self::new_token(TokenType::INT, literal);
                } else {
                    tok = Self::new_token(TokenType::ILLEGAL, &ch);
                }
            }
        }
        self.read_char();
        return tok;
    }

    fn is_letter(ch: char) -> bool {
        'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
    }

    fn read_identifier(&mut self) -> &str {
      let pos = self.position;
      while Self::is_letter(self.ch as char) {
        self.read_char();
      }
      return &self.input[pos..self.position];
    }

    fn read_digit(&mut self) -> &str { 
      let pos = self.position;
      // let ch = self.ch;
      while self.is_digit() {
        self.read_char();
      }

      return &self.input[pos..self.position];
    }

    fn is_digit(&self) -> bool {
      return self.ch.is_ascii_digit();
    }

    fn skip_whitespaces(&mut self) {
        let mut ch = self.ch as char;
        while ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' {
            self.read_char();
            ch = self.ch as char;
        }
    }

    fn new_token(token_type: TokenType, literal: &str) -> Token {
        Token {
            token_type,
            literal: literal.to_string()
        }
    }

    /*
     * This function reads a character from the lexer's input
     * and advances the pointers
     * */
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;  // ASCII FOR "NUL"
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
        // dbg!(&self);
    }

    fn peek_char(&self) -> char {
      if self.read_position >= self.input.len() {
        return '\0';
      }
        return self.input.as_bytes()[self.read_position] as char;
    }

}

#[cfg(test)]
mod tests {
    use crate::token::{self, TokenType};
    use super::*;

    struct TokenRep {
        expected_type: TokenType,
        expected_literal: String,
    }
    impl TokenRep {
        fn new(et: TokenType, el: &str) -> Self {
            Self {
                expected_type: et,
                expected_literal: el.to_string(),
            }
        }
    }

    #[test]
    fn test_next_token() {
        let input = "=+(){}let fn,;five-!*/<> 
        if else return true false == != fn(){}";
        let tests = [
            TokenRep::new(TokenType::ASSIGN, "="),
            TokenRep::new(TokenType::PLUS, "+"),
            TokenRep::new(TokenType::LPAREN, "("),
            TokenRep::new(TokenType::RPAREN, ")"),
            TokenRep::new(TokenType::LBRACE, "{"),
            TokenRep::new(TokenType::RBRACE, "}"),
            TokenRep::new(TokenType::LET, "let"),
            TokenRep::new(TokenType::FUNCTION, "fn"),
            TokenRep::new(TokenType::COMMA, ","),
            TokenRep::new(TokenType::SEMICOLON, ";"),
            TokenRep::new(TokenType::IDENTIFIER, "five"),
            TokenRep::new(TokenType::MINUS, "-"),
            TokenRep::new(TokenType::BANG, "!"),
            TokenRep::new(TokenType::ASTERISK, "*"),
            TokenRep::new(TokenType::SLASH, "/"),
            TokenRep::new(TokenType::LT, "<"),
            TokenRep::new(TokenType::GT, ">"),
            TokenRep::new(TokenType::IF, "if"),
            TokenRep::new(TokenType::ELSE, "else"),
            TokenRep::new(TokenType::RETURN, "return"),
            TokenRep::new(TokenType::TRUE, "true"),
            TokenRep::new(TokenType::FALSE, "false"),
            TokenRep::new(TokenType::EQ, "=="),
            TokenRep::new(TokenType::NOTEQ, "!="),
            TokenRep::new(TokenType::FUNCTION, "fn"),
            TokenRep::new(TokenType::LPAREN, "("),
            TokenRep::new(TokenType::RPAREN, ")"),
            TokenRep::new(TokenType::LBRACE, "{"),
            TokenRep::new(TokenType::RBRACE, "}"),
            TokenRep::new(TokenType::EOF, ""),
        ];
        let mut l = Lexer::new(input);

        for TokenRep { expected_type, expected_literal } in tests
        {
            let tok = l.next_token();
            assert!(
              (tok.token_type == expected_type) && (tok.literal == expected_literal), 
              "\nexpected type = {:?} result = {:?}\nexpected literal = {} result = {}\n", 
              expected_type, tok.token_type, expected_literal, tok.literal);
        }
    }

}

