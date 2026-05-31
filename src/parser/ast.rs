#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

#[derive(Debug)]
pub enum Stmt {
    FunctionDecl(FunctionDecl),
    ExprStatement(Expr),
}

#[derive(Debug)]
pub struct FunctionDecl {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
}

#[derive(Debug)]
pub enum Expr {
    StringLiteral(String),
    FunctionCall(FunctionCall),
}

#[derive(Debug)]
pub struct FunctionCall {
    pub name: String,
    pub args: Vec<Expr>,
}
