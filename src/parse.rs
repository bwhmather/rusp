#[deriving(Clone, PartialEq, Eq, Show)]
pub enum Token<'src> {
    LBrace,
    RBrace,
    Symbol(&'src str),
    Quote,
    QuasiQuote,
    Unquote,
    Unpack,
    EOF,
}

#[deriving(Clone, PartialEq, Eq, Show)]
pub struct TokenizerError;

pub type TokenizerResult<'src> = Result<Token<'src>, TokenizerError>;

pub struct Tokenizer<'src> {
    input: &'src str,
    cursor: uint,
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

    fn next_token(&mut self) -> TokenizerResult<'src> {
        while is_whitespace(self.peek_char()) {
            self.pop_char();
        }

        let c = match self.peek_char() { Some(c) => c, None => return Ok(EOF) };

        match c {
            '(' => { self.pop_char(); return Ok(LBrace); },
            ')' => { self.pop_char(); return Ok(RBrace); },
            '\'' => { self.pop_char(); return Ok(Quote); },
            '`' => { self.pop_char(); return Ok(QuasiQuote); },
            ',' => { self.pop_char(); return Ok(Unquote); },
            '@' => { self.pop_char(); return Ok(Unpack); },
            'a'..'z' | 'A'..'Z' | ':' | '_' => {
                let symbol_start = self.cursor;
                self.pop_char();

                while is_symbol_body(self.peek_char()) {
                    self.pop_char();
                }

                return Ok(Symbol(self.input.slice(symbol_start, self.cursor)));
            }
            _ => return Err(TokenizerError)
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
        assert_eq!(tokenizer.next_token(), Ok(LBrace));
        assert_eq!(tokenizer.next_token(), Ok(RBrace));
        assert_eq!(tokenizer.next_token(), Ok(LBrace));
        assert_eq!(tokenizer.next_token(), Ok(EOF));
    }

    #[test]
    fn test_symbols() {
        let mut tokenizer = tokenize("aAL_-");
        assert_eq!(tokenizer.next_token(), Ok(Symbol("aAL_-")));

        let mut tokenizer = tokenize("one two");
        assert_eq!(tokenizer.next_token(), Ok(Symbol("one")));
        assert_eq!(tokenizer.next_token(), Ok(Symbol("two")));
    }
}
