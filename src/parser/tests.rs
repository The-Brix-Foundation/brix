#[cfg(test)]
mod tests {
    use crate::lexer::scanner::Scanner;
    use crate::parser::ast::*;
    use crate::parser::parser::Parser;
    fn parse(source: &str) -> Program {
        let tokens = Scanner::new(source).scan_tokens().unwrap();
        Parser::new(tokens).parse().unwrap()
    }
    #[test]
    fn test_hello_world() {
        let program = parse(r#"fn main() { print("Hello, World!"); }"#);
        assert_eq!(program.statements.len(), 1);
        let Stmt::FunctionDecl(func) = &program.statements[0] else {
            panic!("Expected FunctionDecl");
        };
        assert_eq!(func.name, "main");
        assert_eq!(func.params.len(), 0);
        assert_eq!(func.body.len(), 1);
        let Stmt::ExprStatement(Expr::FunctionCall(call)) = &func.body[0] else {
            panic!("Expected FunctionCall");
        };
        assert_eq!(call.name, "print");
        assert_eq!(call.args.len(), 1);
        let Expr::StringLiteral(s) = &call.args[0] else {
            panic!("Expected StringLiteral");
        };
        assert_eq!(s, "Hello, World!");
    }
    #[test]
    fn test_empty_function() {
        let program = parse("fn foo() { }");
        let Stmt::FunctionDecl(func) = &program.statements[0] else {
            panic!("Expected FunctionDecl");
        };
        assert_eq!(func.name, "foo");
        assert_eq!(func.body.len(), 0);
    }
}
