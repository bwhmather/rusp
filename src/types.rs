#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    LBracket,
    RBracket,
    Symbol(String),
    Int(i64),
    Str(String),
}

#[derive(PartialEq, Clone, Debug)]
pub enum Expression {
    List(Vec<Expression>),
    Int(i64),
    Str(String),
    Symbol(String),
}


