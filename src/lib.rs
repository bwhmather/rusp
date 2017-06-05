pub mod types;
pub mod lexer;
pub mod parser;

use std::collections::HashMap;
use std::rc::Rc;

use types::Expression;


#[derive(PartialEq, Clone, Debug)]
pub struct Environment {
    parent: Option<Rc<Closure>>,
    bindings: HashMap<String, Value>,
}

impl Environment {
    fn new() -> Self {
        Environment {
            parent: None,
            bindings: HashMap::new(),
        }
    }
}


#[derive(PartialEq, Clone, Debug)]
pub enum Value {
    Int(i64),
    Str(Rc<String>),
    List(Rc<Vec<Value>>),
}


fn op_add(args: &[Value]) -> Value {
    if let (&Value::Int(a), &Value::Int(b)) = (&args[0], &args[1]) {
        return Value::Int(a + b);
    } else {
        panic!("missing argument");
    }
}


fn eval(env: &mut Environment, expr: &Expression) -> Value {
    match expr {
        &Expression::List(ref elements) => {
            match elements[0] {
                Expression::Symbol(ref name) => {
                    match name.as_ref() {
                        "if" => {
                            unimplemented!();
                        }
                        "define" => {
                            unimplemented!();
                        }
                        "+" => {
                            let args: Vec<Value> =
                                elements[1..].iter().map(
                                    |arg| {eval(env, arg)}
                                ).collect();
                            op_add(&args)
                        }
                        _ => {
                            unimplemented!();
                        }
                    }
                }
                _ => {
                    unimplemented!();
                }
            }
        }
        &Expression::Int(i) => {
            return Value::Int(i);

        }
        &Expression::Symbol(ref name) => {
            unimplemented!();
        }
        _ => {
            unimplemented!();
        }
    }
}


#[cfg(test)]
mod tests {
    use lexer::tokenize;
    use parser::read;
    use {Value, Environment};
    use eval;

    #[test]
    fn test_add() {
        let expr = read(tokenize("(+ 1 1)").unwrap()).unwrap();
        let mut env = Environment::new();

        assert_eq!(
            eval(&mut env, &expr),
            Value::Int(2)
        )
    }
}
