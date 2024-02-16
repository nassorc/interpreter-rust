pub mod token;
// use crate::token;
#[derive(Debug)]
pub struct Lexer {
    pub input: String,
    pub ch: char,
    pub position: usize,
    pub read_position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut l = Self {
            input: input.to_string(),
            ch: 0 as char,
            position: 0,
            read_position: 0,
        };
        l.read_char();
        return l;
    }

    pub fn next_token(&mut self) -> token::Token {
        let mut tmp = [0; 4];
        let tk: token::Token;

        self.skip_whitespace();

        tk = match self.ch {
            '+' => self.new_token(token::PLUS, self.ch.encode_utf8(&mut tmp)),
            '-' => self.new_token(token::MINUS, self.ch.encode_utf8(&mut tmp)),
            '*' => self.new_token(token::ASTERISK, self.ch.encode_utf8(&mut tmp)),
            '/' => self.new_token(token::SLASH, self.ch.encode_utf8(&mut tmp)),
            ';' => self.new_token(token::SEMICOLON, self.ch.encode_utf8(&mut tmp)),
            ',' => self.new_token(token::COMMA, self.ch.encode_utf8(&mut tmp)),
            '(' => self.new_token(token::LPAREN, self.ch.encode_utf8(&mut tmp)),
            ')' => self.new_token(token::RPAREN, self.ch.encode_utf8(&mut tmp)),
            '{' => self.new_token(token::LBRACE, self.ch.encode_utf8(&mut tmp)),
            '}' => self.new_token(token::RBRACE, self.ch.encode_utf8(&mut tmp)),
            '<' => self.new_token(token::LT, self.ch.encode_utf8(&mut tmp)),
            '>' => self.new_token(token::GT, self.ch.encode_utf8(&mut tmp)),
            '\0' => self.new_token(token::EOF, self.ch.encode_utf8(&mut tmp)),
            '=' => {
                if self.peek_char_is('=') {
                    self.read_char();
                    self.new_token(token::EQ, "==")
                } else {
                    self.new_token(token::ASSIGN, self.ch.encode_utf8(&mut tmp))
                }
            }
            '!' => {
                if self.peek_char_is('=') {
                    self.read_char();
                    self.new_token(token::NOTEQ, "!=")
                } else {
                    self.new_token(token::BANG, self.ch.encode_utf8(&mut tmp))
                }
            }
            _ => {
                if self.is_letter() {
                    let identifier = self.read_identifier();
                    let token_type = token::get_identifier(&identifier);
                    return self.new_token(token_type, &identifier);
                } else if self.is_digit() {
                    let number = self.read_number();
                    return self.new_token(token::INT, &number);
                }
                token::Token::new(token::ILLEGAL, ('\0' as char).encode_utf8(&mut tmp))
            }
        };
        self.read_char();
        return tk;
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0 as char;
        } else {
            self.ch = self.input.as_bytes()[self.read_position] as char;
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> String {
        let pos = self.position;
        while self.is_letter() {
            self.read_char();
        }
        self.input[pos..self.position].to_owned()
    }

    fn read_number(&mut self) -> String {
        let pos = self.position;
        while self.is_digit() {
            self.read_char();
        }
        self.input[pos..self.position].to_owned()
    }

    fn new_token(&self, token_type: token::TokenType, literal: &str) -> token::Token {
        token::Token::new(token_type, literal)
    }

    fn peek_char(&self) -> char {
        if self.read_position <= self.input.len() {
            return self.input.as_bytes()[self.read_position] as char;
        }
        return '\0';
    }

    fn peek_char_is(&self, ch: char) -> bool {
        self.peek_char() == ch
    }

    fn is_letter(&self) -> bool {
        ('a' <= self.ch && self.ch <= 'z') || ('A' <= self.ch && self.ch <= 'Z') || (self.ch == '_')
    }

    fn is_digit(&self) -> bool {
        ('0' <= self.ch && self.ch <= '9')
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }
}

#[cfg(test)]
mod test {
    use self::token;

    use super::*;

    struct ExpectedToken {
        token: token::TokenType,
        literal: String,
    }

    #[test]
    fn test_lexer_parsing_completeness() {
        let input = "
        +-*/
        ,;
        !<>= == !=
        () {}
        fn let return 
        myVar 10
        ";

        let tests: Vec<ExpectedToken> = vec![
            ExpectedToken {
                token: token::PLUS,
                literal: "+".to_string(),
            },
            ExpectedToken {
                token: token::MINUS,
                literal: "-".to_string(),
            },
            ExpectedToken {
                token: token::ASTERISK,
                literal: "*".to_string(),
            },
            ExpectedToken {
                token: token::SLASH,
                literal: "/".to_string(),
            },
            ExpectedToken {
                token: token::COMMA,
                literal: ",".to_string(),
            },
            ExpectedToken {
                token: token::SEMICOLON,
                literal: ";".to_string(),
            },
            ExpectedToken {
                token: token::BANG,
                literal: "!".to_string(),
            },
            ExpectedToken {
                token: token::LT,
                literal: "<".to_string(),
            },
            ExpectedToken {
                token: token::GT,
                literal: ">".to_string(),
            },
            ExpectedToken {
                token: token::ASSIGN,
                literal: "=".to_string(),
            },
            ExpectedToken {
                token: token::EQ,
                literal: "==".to_string(),
            },
            ExpectedToken {
                token: token::NOTEQ,
                literal: "!=".to_string(),
            },
            ExpectedToken {
                token: token::LPAREN,
                literal: "(".to_string(),
            },
            ExpectedToken {
                token: token::RPAREN,
                literal: ")".to_string(),
            },
            ExpectedToken {
                token: token::LBRACE,
                literal: "{".to_string(),
            },
            ExpectedToken {
                token: token::RBRACE,
                literal: "}".to_string(),
            },
            ExpectedToken {
                token: token::FUNCTION,
                literal: "fn".to_string(),
            },
            ExpectedToken {
                token: token::LET,
                literal: "let".to_string(),
            },
            ExpectedToken {
                token: token::RETURN,
                literal: "return".to_string(),
            },
            ExpectedToken {
                token: token::IDENTIFIER,
                literal: "myVar".to_string(),
            },
            ExpectedToken {
                token: token::INT,
                literal: "10".to_string(),
            },
            ExpectedToken {
                token: token::EOF,
                literal: "\0".to_string(),
            },
        ];

        let mut lexer = Lexer::new(input);

        for tt in tests {
            let tk = lexer.next_token();
            assert_eq!(tt.token, tk.token_type);
            assert_eq!(tt.literal, tk.literal);
        }
    }
}
