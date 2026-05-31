#[cfg(test)]
mod tests {
    use crate::lexer::scanner::Scanner;
    use crate::lexer::token::Token;

    #[test]
    fn test_hello_world() {
        let src = r#"fn main() { print("Hello, World!"); }"#;
        let tokens = Scanner::new(src).scan_tokens().unwrap();

        assert_eq!(tokens[0], Token::Fn);
        assert_eq!(tokens[1], Token::Identifier("main".to_string()));
        assert_eq!(tokens[2], Token::LParen);
        assert_eq!(tokens[3], Token::RParen);
        assert_eq!(tokens[4], Token::LBrace);
        assert_eq!(tokens[5], Token::Identifier("print".to_string()));
        assert_eq!(tokens[6], Token::LParen);
        assert_eq!(tokens[7], Token::StringLiteral("Hello, World!".to_string()));
        assert_eq!(tokens[8], Token::RParen);
        assert_eq!(tokens[9], Token::Semicolon);
        assert_eq!(tokens[10], Token::RBrace);
        assert_eq!(tokens[11], Token::EOF);
    }
}
