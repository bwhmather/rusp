use std::str::CharRange;

#[deriving(Clone, PartialEq, Eq, Show)]
pub enum Token<'src> {
    LBrace,
    RBrace,
    Symbol(&'src str),
}

pub struct Tokenizer<'src> {
    input: &'src str,
    cursor: uint,
}

impl<'src> Iterator<Token<'src>> for Tokenizer<'src> {
    #[inline]
    fn next(&mut self) -> Option<Token<'src>> {
        let mut token_start = self.cursor;
        let CharRange {ch, next: mut token_stop} =
            self.input.char_range_at(token_start);
        let token = match self.input.char_at(token_start) {
            '(' => Some(LBrace),
            ')' => Some(RBrace),
            'a'..'z' | 'A'..'Z' | ':' | '_' => {
                let value = self.input.slice_from(token_start);
                for (idx, ch) in value.char_indices() {
                    token_stop = token_start + idx;
                    match ch {
                        'a'..'z' | 'A'..'Z' | ':' | '_' => (),
                        _ => break
                    }
                };
                Some(Symbol(self.input.slice(token_start, token_stop)))
            }
            _ => None
        };
        self.cursor = token_stop;
        return token;
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
        assert_eq!(tokenizer.next(), Some(LBrace));
        assert_eq!(tokenizer.next(), Some(RBrace));
        assert_eq!(tokenizer.next(), Some(LBrace));

    }
}
