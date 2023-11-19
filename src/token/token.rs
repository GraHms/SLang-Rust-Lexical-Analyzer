#[derive(Debug, PartialEq)]
pub enum TokenType {
    ILLEGAL,
    EOF,
    // Identifiers + literals
    IDENT, // add, foobar, x, y, ...
    INT,   // 1343456
    FLOAT, // for floating-point numbers
    // Operators
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    EQ,
    NotEq,
    LT,
    GT,
    // Delimiters
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    LBRACKET,
    RBRACKET,
    // Keywords
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
    STRING,
    COLON,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub r#type: TokenType,
    pub literal: String,
    pub line: i32,
    pub column: i32,
}


pub fn lookup_ident(ident: &str) -> TokenType {
    match ident {
        "fn" => TokenType::FUNCTION,
        "func" => TokenType::FUNCTION,
        "def" => TokenType::FUNCTION,
        "let" => TokenType::LET,
        "true" => TokenType::TRUE,
        "false" => TokenType::FALSE,
        "if" => TokenType::IF,
        "else" => TokenType::ELSE,
        "return" => TokenType::RETURN,
        _ => TokenType::IDENT,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_lookup_ident() {
        assert_eq!(lookup_ident("let"), TokenType::LET);
        assert_eq!(lookup_ident("if"), TokenType::IF);
        assert_eq!(lookup_ident("foobar"), TokenType::IDENT);
    }

    #[test]
    fn test_should_match_token_equality() {
        let token1 = Token {
            r#type: TokenType::IDENT,
            literal: String::from("foo"),
            line: 1,
            column: 2,
        };

        let token2 = Token {
            r#type: TokenType::IDENT,
            literal: String::from("foo"),
            line: 1,
            column: 2,
        };

        assert_eq!(token1, token2);
    }
}
