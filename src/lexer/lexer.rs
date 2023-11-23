use crate::token::token;
use crate::token::token::{Token, TokenType};

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
    line: i32,
    column: i32,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
            line: 1,
            column: 1,
        };
        lexer.read_char();
        lexer
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let mut tok = Token {
            r#type: TokenType::ILLEGAL,
            literal: String::from(""),
            line: self.line,
            column: self.column,
        };

        match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let literal = format!("{}{}", ch, self.ch);
                    tok = Token {
                        r#type: TokenType::EQ,
                        literal,
                        line: self.line,
                        column: self.column,
                    };
                } else {
                    tok = Token {
                        r#type: TokenType::ASSIGN,
                        literal: self.ch.to_string(),
                        line: self.line,
                        column: self.column,
                    };
                }
            }
            '+' => {
                tok = Token {
                    r#type: TokenType::PLUS,
                    literal: self.ch.to_string(),
                    line: self.line,
                    column: self.column,
                };
            }
            '(' => {
                tok = Token {
                    r#type: TokenType::LPAREN,
                    literal: self.ch.to_string(),
                    line: self.line,
                    column: self.column,
                };
            }
            '-' => {
                tok = Token {
                    r#type: TokenType::MINUS,
                    literal: self.ch.to_string(),
                    line: self.line,
                    column: self.column,
                };
            }
            '*' => {
                tok = Token {
                    r#type: TokenType::ASTERISK,
                    literal: self.ch.to_string(),
                    line: self.line,
                    column: self.column,
                };
            }
            '/' => {
                if self.peek_char() == '/' {
                    self.read_char();
                    self.skip_comment();
                    return self.next_token();
                } else {
                    tok = Token {
                        r#type: TokenType::SLASH,
                        literal: self.ch.to_string(),
                        line: self.line,
                        column: self.column,
                    };
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let literal = format!("{}{}", ch, self.ch);
                    tok = Token {
                        r#type: TokenType::NotEq,
                        literal,
                        line: self.line,
                        column: self.column,
                    };
                } else {
                    tok = Token {
                        r#type: TokenType::BANG,
                        literal: self.ch.to_string(),
                        line: self.line,
                        column: self.column,
                    };
                }
            }
            '<' => {
                tok = Token {
                    r#type: TokenType::LT,
                    literal: self.ch.to_string(),
                    line: self.line,
                    column: self.column,
                };
            }
            '>' => {
                tok = Token {
                    r#type: TokenType::GT,
                    literal: self.ch.to_string(),
                    line: self.line,
                    column: self.column,
                };
            }
            ',' => {
                tok = Token {
                    r#type: TokenType::COMMA,
                    literal: self.ch.to_string(),
                    line: self.line,
                    column: self.column,
                };
            }
            ';' => {
                tok = Token {
                    r#type: TokenType::SEMICOLON,
                    literal: self.ch.to_string(),
                    line: self.line,
                    column: self.column,
                };
            }
            ')' => {
                tok = Token {
                    r#type: TokenType::RPAREN,
                    literal: self.ch.to_string(),
                    line: self.line,
                    column: self.column,
                };
            }
            '{' => {
                tok = Token {
                    r#type: TokenType::LBRACE,
                    literal: self.ch.to_string(),
                    line: self.line,
                    column: self.column,
                };
            }
            '}' => {
                tok = Token {
                    r#type: TokenType::RBRACE,
                    literal: self.ch.to_string(),
                    line: self.line,
                    column: self.column,
                };
            }
            '[' => {
                tok = Token {
                    r#type: TokenType::LBRACKET,
                    literal: self.ch.to_string(),
                    line: self.line,
                    column: self.column,
                };
            }
            ']' => {
                tok = Token {
                    r#type: TokenType::RBRACKET,
                    literal: self.ch.to_string(),
                    line: self.line,
                    column: self.column,
                };
            }
            ':' => {
                tok = Token {
                    r#type: TokenType::COLON,
                    literal: self.ch.to_string(),
                    line: self.line,
                    column: self.column,
                };
            }
            '"' => {
                tok.r#type = TokenType::STRING;
                tok.literal = self.read_string();
            }
            '0'..='9' => {
                tok.literal = self.read_number();
                if tok.literal.contains('.') {
                    tok.r#type = TokenType::FLOAT;
                } else {
                    tok.r#type = TokenType::INT;
                }
                return tok;
            }


            _ => {
                if self.is_letter(self.ch) {
                    tok.literal = self.read_identifier();
                    tok.r#type = token::lookup_ident(&tok.literal.clone());
                    return tok;
                } else if self.is_digit(self.ch) {
                    tok.literal = self.read_number();
                    if tok.literal.contains('.') {
                        tok.r#type = TokenType::FLOAT;
                    } else {
                        tok.r#type = TokenType::INT;
                    }
                    return tok;
                } else {
                    tok = Token {
                        r#type: TokenType::ILLEGAL,
                        literal: self.ch.to_string(),
                        line: self.line,
                        column: self.column,
                    };
                }
            }
        }

        self.read_char();

        tok
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    fn is_letter(&self, ch: char) -> bool {
        (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z') || ch == '_'
    }

    fn is_digit(&self, ch: char) -> bool {
        (ch >= '0' && ch <= '9') || ch == '.'
    }
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
        if self.ch == '\n' {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input.chars().nth(self.read_position).unwrap()
        }
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.is_letter(self.ch) {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.is_digit(self.ch) {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }
    fn skip_comment(&mut self) {
        while self.ch != '\n' && self.ch != '\0' {
            self.read_char();
        }
    }
    fn read_string(&mut self) -> String {
        let position = self.position + 1; // Skip the opening double quote
        while self.ch != '"' && self.ch != '\0' {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_get_next_token() {
        let input = "let x = 5;";
        let mut lexer = Lexer::new(input.to_string());

        let expected_tokens = vec![
            Token {
                r#type: TokenType::LET,
                literal: "let".to_string(),
                line: 1,
                column: 1,
            },
            Token {
                r#type: TokenType::IDENT,
                literal: "x".to_string(),
                line: 1,
                column: 5,
            },
            Token {
                r#type: TokenType::ASSIGN,
                literal: "=".to_string(),
                line: 1,
                column: 7,
            },
            Token {
                r#type: TokenType::INT,
                literal: "5".to_string(),
                line: 1,
                column: 9,
            },
            Token {
                r#type: TokenType::SEMICOLON,
                literal: ";".to_string(),
                line: 1,
                column: 10,
            },
        ];

        for expected_token in expected_tokens {
            let token = lexer.next_token();
            println!("Actual:   {:?}", token.r#type);
            println!("Expected: {:?}", expected_token.r#type);

            assert_eq!(token.r#type, expected_token.r#type);
            assert_eq!(token.literal, expected_token.literal);

        }


    }
}
