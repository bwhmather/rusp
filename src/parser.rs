use std::result;
use std::iter::Peekable;

use lexer::{Lexer, Token};
use lexer;

pub use self::Expression::*;


#[derive(PartialEq, Clone, Debug)]
pub enum Expression {
    List(Vec<Expression>),
    Int(i64),
    Str(String),
    Symbol(String),
}


type ParserError = &'static str;
type Result<T> = result::Result<T, ParserError>;


fn read_from_peekable<I>(tokens: &mut Peekable<I>) -> Result<Expression>
where I: Iterator<Item = Token> {

    if let Some(token) = tokens.next() {
        match token {
            Token::LBracket => {
                let mut elements = Vec::new();

                loop {
                    match tokens.peek() {
                        Some(&Token::RBracket) => {
                            break;
                        }
                        None => {
                            return Err("unexpected end of input");
                        }
                        _ => {}
                    }

                    elements.push(read_from_peekable(tokens).unwrap());
                }
                return Ok(Expression::List(elements));
            }
            Token::RBracket => {
                return Err("mismatched )");
            }
            Token::Int(i) => {
                return Ok(Expression::Int(i));
            }
            _ => {
                return Err("unrecognized");
            }
        }
    } else {
        Err("empty")
    }
}

pub fn read<I>(tokens: I) -> Result<Expression>
where I: IntoIterator<Item = Token> {
    let mut tokens = tokens.into_iter().peekable();

    return read_from_peekable(&mut tokens);
}


#[cfg(test)]
mod tests {
    use lexer::tokenize;
    use parser::{Expression, read};

    #[test]
    fn test_simple() {
        let tokens = tokenize(
            "(define circle-area (lambda (r) (* pi (* r r))))"
        ).unwrap();

        assert_eq!(
            read(tokens).unwrap(),
            Expression::List(vec![
                Expression::Symbol(String::from("circle-area")),
                Expression::List(vec![
                    Expression::Symbol(String::from("lambda")),
                    Expression::List(vec![
                        Expression::Symbol(String::from("r"))
                    ]),
                    Expression::List(vec![
                        Expression::Symbol(String::from("*")),
                        Expression::Symbol(String::from("pi")),
                        Expression::List(vec![
                            Expression::Symbol(String::from("r")),
                            Expression::Symbol(String::from("r"))
                        ])
                    ]),
                ])
            ])
        );
    }
}





//
//
//pub fn read() {
//    let mut parser = Parser::new();
//
//    for token in tokens {
//
//
//        match parser.push(token) {
//            More(parser) => {
//                parser,
//            }
//            Done(expression) => return expression,
//        }
//    }
//
//    Err("unexpexted EOF")
//}
//
//pub fn parse(input: &str) -> Result<Expression> {
//    unimplemented!();
//}
