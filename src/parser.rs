use std::result;
use std::iter::Peekable;

use types::{Token, Expression};
use lexer::Lexer;
use lexer;


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
                            tokens.next();
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
            Token::Symbol(s) => {
                return Ok(Expression::Symbol(s));
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
    use types::Expression;
    use lexer::tokenize;
    use parser::read;

    #[test]
    fn test_symbol() {
        let tokens = tokenize(
            "symbol"
        ).unwrap();

        assert_eq!(
            read(tokens).unwrap(),
            Expression::Symbol(String::from("symbol"))
        )
    }


    #[test]
    fn test_simple() {
        let tokens = tokenize(
            "(define circle-area (lambda (r) (* pi (* r r))))"
        ).unwrap();

        assert_eq!(
            read(tokens).unwrap(),
            Expression::List(vec![
                Expression::Symbol(String::from("define")),
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
                            Expression::Symbol(String::from("*")),
                            Expression::Symbol(String::from("r")),
                            Expression::Symbol(String::from("r"))
                        ])
                    ]),
                ])
            ])
        );
    }
}
