pub mod types;
pub mod lexer;
pub mod parser;

use std::collections::HashMap;
use std::rc::Rc;

use types::Expression;


#[derive(PartialEq, Clone, Debug)]
pub struct Environment {
    bindings: HashMap<String, Value>,
}

impl Environment {
    fn new() -> Self {
        Environment {
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


fn eval(env: &Environment, expr: &Expression) -> Value {
    match expr {
        &Expression::List(ref elements) => {
            match elements[0] {
                Expression::Symbol(ref name) => {
                    match name.as_ref() {
                        "if" => {
                            unimplemented!();
                        }
                        "let" => {
                            let mut scope = env.clone();

                            let bindings = elements[1].expect_list();
                            for binding_expr in bindings {
                                let binding = binding_expr.expect_list();

                                let binding_name = binding[0].expect_symbol();
                                let binding_value = eval(env, &binding[1]);

                                scope.bindings.insert(
                                    String::from(binding_name), binding_value,
                                );
                            }

                            eval(&scope, &elements[2])
                        }
                        "+" => {
                            let args: Vec<Value> =
                                elements[1..].iter().map(
                                    |arg| {eval(env, arg)}
                                ).collect();
                            op_add(&args)
                        }
                        _ => {
                            env.bindings.get(name).unwrap().clone()
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
            env.bindings.get(name).unwrap().clone()
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
        let env = Environment::new();

        assert_eq!(
            eval(&env, &expr),
            Value::Int(2)
        )
    }

    #[test]
    fn test_let() {
        let expr = read(tokenize(
            "(let ((a 1) (b 2)) (+ a b))"
        ).unwrap()).unwrap();
        let env = Environment::new();

        assert_eq!(
            eval(&env, &expr),
            Value::Int(3)
        );
    }
}
