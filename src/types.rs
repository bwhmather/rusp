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


impl Expression {
    pub fn expect_list(&self) -> &[Expression] {
        match self {
            &Expression::List(ref vec) => vec.as_slice(),
            _ => panic!(),
        }
    }

    pub fn expect_int(&self) -> i64 {
        match self {
            &Expression::Int(i) => i,
            _ => panic!(),
        }
    }

    pub fn expect_str(&self) -> &str {
        match self {
            &Expression::Str(ref string) => string.as_ref(),
            _ => panic!(),
        }
    }

    pub fn expect_symbol(&self) -> &str {
        match self {
            &Expression::Symbol(ref string) => string.as_ref(),
            _ => panic!(),
        }
    }
}


//#[derive(PartialEq, Clone, Debug)]
//pub enum Value {
//    Int(i64),
//    Str(Rc<String>),
//    List(Rc<Vec<Value>),
//    // Closure(Rc<Closure>),
//    Builtin(fn(Value[]) -> Value),
//}
