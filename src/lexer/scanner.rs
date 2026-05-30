use crate::lexer::token::Token;

pub struct Scanner {
    source: Vec<char>,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Scanner {
            source: source.chars().collect(),
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            if let Some(token) = self.next_token()? {
                tokens.push(token);
            }
        }

        tokens.push(Token::EOF);
        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Option<Token>, String> {
        let c = self.advance();

        match c {
            ' ' | '\r' | '\t' => Ok(None),

            '\n' => {
                self.line += 1;
                Ok(None)
            }

            '(' => Ok(Some(Token::LParen)),
            ')' => Ok(Some(Token::RParen)),
            '{' => Ok(Some(Token::LBrace)),
            '}' => Ok(Some(Token::RBrace)),
            ';' => Ok(Some(Token::Semicolon)),

            '"' => Ok(Some(self.string()?)),

            c if c.is_alphabetic() || c == '_' => Ok(Some(self.identifier(c))),

            c => Err(format!("Line {}: unexpected character '{}'", self.line, c)),
        }
    }

    fn string(&mut self) -> Result<Token, String> {
        let mut value = String::new();

        while !self.is_at_end() && self.peek() != '"' {
            value.push(self.advance());
        }

        if self.is_at_end() {
            return Err(format!("Line {}: unterminated string", self.line));
        }

        self.advance(); // closing "
        Ok(Token::StringLiteral(value))
    }

    fn identifier(&mut self, first: char) -> Token {
        let mut name = String::from(first);

        while !self.is_at_end() && (self.peek().is_alphanumeric() || self.peek() == '_') {
            name.push(self.advance());
        }

        match name.as_str() {
            "fn" => Token::Fn,
            _ => Token::Identifier(name),
        }
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source[self.current]
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
