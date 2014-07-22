#[deriving(Clone, PartialEq, Eq, Show)]
pub enum Token<'src> {
    LBrace,
    RBrace,
    Symbol(&'src str),
}

#[deriving(Clone, PartialEq, Eq, Show)]
pub struct TokenizerError;

pub type TokenizerResult<'src> = Result<Token<'src>, TokenizerError>;

pub struct Tokenizer<'src> {
    input: &'src str,
    cursor: uint,
}

impl<'src> Tokenizer<'src> {
    fn peek_char(&self) -> Option<char> {
        if self.cursor >= self.input.len() {
            return None
        } else {
            return Some(self.input.char_at(self.cursor));
        }
    }

    fn pop_char(&mut self) -> Option<char> {
        if self.cursor >= self.input.len() {
            return None
        } else {
            let range = self.input.char_range_at(self.cursor);
            self.cursor = range.next;
            return Some(range.ch);
        }
    }
}

fn is_symbol_body(c: Option<char>) -> bool {
    let c = match c { Some(c) => c, None => return false };

    return (c >= 'a' && c <= 'z')
        || (c >= 'A' && c <= 'Z')
        || (c >= '0' && c <= '9')
        || (c == '_') || (c == '-');
}

fn is_whitespace(c: Option<char>) -> bool {
    let c = match c { Some(c) => c, None => return false };

    return match c {
        ' ' | '\t' | '\n' => true,
        _ => false
    };
}


impl<'src> Iterator<TokenizerResult<'src>> for Tokenizer<'src> {
    fn next(&mut self) -> Option<TokenizerResult<'src>> {
        while is_whitespace(self.peek_char()) {
            self.pop_char();
        }

        let c = match self.peek_char() { Some(c) => c, None => return None };

        match c {
            '(' => { self.pop_char(); return Some(Ok(LBrace)); },
            ')' => { self.pop_char(); return Some(Ok(RBrace)); },
            'a'..'z' | 'A'..'Z' | ':' | '_' => {
                let symbol_start = self.cursor;
                self.pop_char();

                while is_symbol_body(self.peek_char()) {
                    self.pop_char();
                }

                return Some(Ok(
                    Symbol(self.input.slice(symbol_start, self.cursor))
                ));
            }
            _ => return Some(Err(TokenizerError))
        };
    }
}

pub fn tokenize<'src>(input: &'src str) -> Tokenizer<'src> {
    Tokenizer {input: input, cursor: 0}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_braces() {
        let mut tokenizer = tokenize("()(");
        assert_eq!(tokenizer.next(), Some(Ok(LBrace)));
        assert_eq!(tokenizer.next(), Some(Ok(RBrace)));
        assert_eq!(tokenizer.next(), Some(Ok(LBrace)));
        assert_eq!(tokenizer.next(), None);
    }

    #[test]
    fn test_symbols() {
        let mut tokenizer = tokenize("aAL_-");
        assert_eq!(tokenizer.next(), Some(Ok(Symbol("aAL_-"))));

        let mut tokenizer = tokenize("one two");
        assert_eq!(tokenizer.next(), Some(Ok(Symbol("one"))));
        assert_eq!(tokenizer.next(), Some(Ok(Symbol("two"))));
    }
}
