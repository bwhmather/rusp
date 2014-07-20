use std::str::CharRange;

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
        if (self.cursor >= self.input.len()) {
            return None
        } else {
            return Some(self.input.char_at(self.cursor));
        }
    }

    fn pop_char(&mut self) -> Option<char> {
        if (self.cursor >= self.input.len()) {
            return None
        } else {
            let range = self.input.char_range_at(self.cursor);
            self.cursor = range.next;
            return Some(range.ch);
        }
    }
}



fn symbol_continue(c: Option<char>) -> bool {
    let c = match c { Some(c) => c, None => return false };

    return (c >= 'a' && c <= 'z')
        || (c >= 'A' && c <= 'Z')
        || (c >= '0' && c <= '9')
        || (c == '_') || (c == '-');
}

impl<'src> Iterator<TokenizerResult<'src>> for Tokenizer<'src> {
    #[inline]
    fn next(&mut self) -> Option<TokenizerResult<'src>> {
        let c = match self.peek_char() { Some(c) => c, None => return None };
        match c {
            '(' => { self.pop_char(); return Some(Ok(LBrace)); },
            ')' => { self.pop_char(); return Some(Ok(RBrace)); },
            'a'..'z' | 'A'..'Z' | ':' | '_' => {
                let symbol_start = self.cursor;
                self.pop_char();

                while symbol_continue(self.peek_char()) {
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

#[inline]
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
}
