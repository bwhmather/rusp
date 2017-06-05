use std::result;

use types::Token;
use types::Token::*;


type LexerError = &'static str;
type Result<T> = result::Result<T, LexerError>;


#[derive(Clone, Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    cursor: usize,
}


impl<'a> Lexer<'a> {

    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            input: input,
            cursor: 0,
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.input[self.cursor..].chars().next()
    }

    fn pop_char(&mut self) {
        match self.input[self.cursor..].chars().next() {
            Some(ch) => {
                self.cursor += ch.len_utf8();
            }
            None => {}
        }
    }

    fn scan_number(&mut self) -> Result<Token> {
        let token_start = self.cursor;

        match self.peek_char() {
            Some('-') => self.pop_char(),
            _ => (),
        }

        loop {
            match self.peek_char() {
                Some(ch) => {
                    match ch {
                        '0'...'9' => {
                            self.pop_char();
                        }
                        _ => break,
                    }
                }
                None => break,
            }
        }

        Ok(Int(
            self.input[token_start..self.cursor].parse::<i64>().unwrap()
        ))
    }

    fn scan_string(&mut self) -> Result<Token> {
        unimplemented!();
    }

    fn scan_char(&mut self) -> Result<Token> {
        unimplemented!();
    }

    fn scan_symbol(&mut self) -> Result<Token> {
        let token_start = self.cursor;

        loop {
            match self.peek_char() {
                Some('a'...'z') | Some('+') | Some('-') | Some('*') => {
                    self.pop_char();
                }
                _ => {break;}
            }
        }

        Ok(Symbol(
            self.input[token_start..self.cursor].to_string()
        ))
    }

    fn consume_whitespace(&mut self) {
        loop {
            match self.peek_char() {
                Some(' ') | Some('\n') => {
                    self.pop_char();
                }
                _ => {
                    break;
                }
            }
        }
    }
}


impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Result<Token>> {
        self.consume_whitespace();

        match self.peek_char() {
            Some(ch) => Some(match ch {
                '(' => {
                    self.pop_char();
                    Ok(LBracket)
                }
                ')' => {
                    self.pop_char();
                    Ok(RBracket)
                }
                '0'...'9' | '-' => {
                    self.scan_number()
                }

                '"' => {
                    self.scan_string()
                }

                '\'' => {
                    self.scan_char()
                }
                _ => {
                    self.scan_symbol()
                }
            }),
            None => None,
        }
    }
}


pub fn tokenize(chars: &str) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();

    let lexer = Lexer::new(chars);

    for result in lexer {
        match result {
            Ok(token) => {
                tokens.push(token);
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    return Ok(tokens);
}




#[cfg(test)]
mod tests {
    use lexer::tokenize;
    use types::Token::*;

    #[test]
    fn test_brackets() {
        assert_eq!(
            tokenize("(( () () ))"),
            Ok(vec![
                LBracket, LBracket, LBracket, RBracket,
                LBracket, RBracket, RBracket, RBracket,
            ])
        )
    }

    #[test]
    fn test_numbers() {
        assert_eq!(
            tokenize("1"),
            Ok(vec![Int(1)])
        );

        assert_eq!(
            tokenize("10"),
            Ok(vec![Int(10)])
        );

        assert_eq!(
            tokenize("01"),
            Ok(vec![Int(1)])
        );

        assert_eq!(
            tokenize("-1"),
            Ok(vec![Int(-1)])
        );

        assert_eq!(
            tokenize("-10"),
            Ok(vec![Int(-10)])
        );

        assert_eq!(
            tokenize("-01"),
            Ok(vec![Int(-1)])
        );
    }

    #[test]
    fn test_symbols() {
        assert_eq!(
            tokenize("symbol"),
            Ok(vec![Symbol(String::from("symbol"))])
        );
    }
}
