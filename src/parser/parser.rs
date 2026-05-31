use crate::lexer::token::Token;
use crate::parser::ast::*;
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }
    pub fn parse(&mut self) -> Result<Program, String> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.parse_stmt()?);
        }
        Ok(Program { statements })
    }
    fn parse_stmt(&mut self) -> Result<Stmt, String> {
        if self.check(Token::Fn) {
            Ok(Stmt::FunctionDecl(self.parse_function()?))
        } else {
            let expr = self.parse_expr()?;
            self.expect(Token::Semicolon, "Expected ';' after expression")?;
            Ok(Stmt::ExprStatement(expr))
        }
    }
    fn parse_function(&mut self) -> Result<FunctionDecl, String> {
        self.expect(Token::Fn, "Expected 'fn'")?;
        let name = self.expect_identifier("Expected function name")?;
        self.expect(Token::LParen, "Expected '(' after function name")?;
        self.expect(Token::RParen, "Expected ')'")?;
        self.expect(Token::LBrace, "Expected '{' before function body")?;
        let mut body = Vec::new();
        while !self.check(Token::RBrace) && !self.is_at_end() {
            body.push(self.parse_stmt()?);
        }
        self.expect(Token::RBrace, "Expected '}' after function body")?;
        Ok(FunctionDecl {
            name,
            params: vec![],
            body,
        })
    }
    fn parse_expr(&mut self) -> Result<Expr, String> {
        if let Some(name) = self.match_identifier() {
            if self.check(Token::LParen) {
                return Ok(Expr::FunctionCall(self.parse_call(name)?));
            }
            return Err(format!("Unexpected identifier '{}'", name));
        }
        if let Some(s) = self.match_string() {
            return Ok(Expr::StringLiteral(s));
        }
        Err(format!("Unexpected token: {:?}", self.peek()))
    }
    fn parse_call(&mut self, name: String) -> Result<FunctionCall, String> {
        self.expect(Token::LParen, "Expected '('")?;
        let mut args = Vec::new();
        if !self.check(Token::RParen) {
            args.push(self.parse_expr()?);
            while self.match_token(Token::Comma) {
                args.push(self.parse_expr()?);
            }
        }
        self.expect(Token::RParen, "Expected ')' after arguments")?;
        Ok(FunctionCall { name, args })
    }
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
    fn check(&self, token: Token) -> bool {
        if self.is_at_end() {
            return false;
        }
        std::mem::discriminant(self.peek()) == std::mem::discriminant(&token)
    }
    fn match_token(&mut self, token: Token) -> bool {
        if self.check(token) {
            self.advance();
            true
        } else {
            false
        }
    }
    fn expect(&mut self, token: Token, msg: &str) -> Result<(), String> {
        if self.check(token) {
            self.advance();
            Ok(())
        } else {
            Err(format!("{}, got {:?}", msg, self.peek()))
        }
    }
    fn expect_identifier(&mut self, msg: &str) -> Result<String, String> {
        if let Token::Identifier(name) = self.peek().clone() {
            self.advance();
            Ok(name)
        } else {
            Err(format!("{}, got {:?}", msg, self.peek()))
        }
    }
    fn match_identifier(&mut self) -> Option<String> {
        if let Token::Identifier(name) = self.peek().clone() {
            self.advance();
            Some(name)
        } else {
            None
        }
    }
    fn match_string(&mut self) -> Option<String> {
        if let Token::StringLiteral(s) = self.peek().clone() {
            self.advance();
            Some(s)
        } else {
            None
        }
    }
    fn is_at_end(&self) -> bool {
        matches!(self.peek(), Token::EOF)
    }
}
