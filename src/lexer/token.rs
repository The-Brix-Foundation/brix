#[derive(Debug, Clone, PartialEq)]
pub enum FStringPart {
    String(String),
    Expression(Vec<Token>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    Integer(i64),
    Float(f64),
    StringLiteral(String),
    FString(Vec<FStringPart>),
    Bool(bool),
    Char(char),

    // Keywords
    Let,
    Mut,
    Const,
    Fn,
    Return,
    If,
    Else,
    Match,
    For,
    While,
    Loop,
    Break,
    Continue,
    In,
    Struct,
    Enum,
    Mod,
    Use,
    Pub,
    Owned,
    Borrow,
    Share,
    Copy,
    Move,
    Task,
    Async,
    Await,
    Spawn,
    Blueprint,
    Contract,
    Effect,
    Decorator,
    Try,
    True,
    False,
    As,
    Type,
    Op,
    Pure,
    SelfKw,

    // Identifiers
    Identifier(String),

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Eq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    And,
    Or,
    Not,
    Assign,
    PlusAssign,
    MinusAssign,
    StarAssign,
    SlashAssign,
    Bang,
    Question,
    Arrow,      // ->
    FatArrow,   // =>
    ColonColon, // ::
    DotDot,     // ..
    DotDotEq,   // ..=

    // Delimiters
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Dot,
    Colon,
    Semicolon,
    At,             // @
    LBracketEffect, // [ for effects
    Pipe,           // |  for closures

    // Special
    EOF,
}
