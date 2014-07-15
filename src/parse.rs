use std::str::CharRange;

pub enum Token<'src> {
    LBrace,
    RBrace,
    Symbol(&'src str),
}

pub struct Tokenizer<'src> {
    input: &'src str,
}

impl<'src> Iterator<Token<'src>> for Tokenizer<'src> {
    #[inline]
    fn next(&mut self) -> Option<Token<'src>> {
        let CharRange {ch, next: mut next_token} = self.input.char_range_at(0);
        let token = match ch {
            '(' => Some(LBrace),
            ')' => Some(RBrace),
            'a'..'z' | 'A'..'Z' | ':' | '_' => {
                for (idx, ch) in self.input.char_indices() {
                    next_token = idx;
                    match ch {
                        'a'..'z' | 'A'..'Z' | ':' | '_' => (),
                        _ => break
                    }
                };
                Some(Symbol(self.input.slice_to(next_token)))
            }
            _ => None
        };
        self.input = self.input.slice_from(next_token);
        return token;
    }
}

#[inline]
pub fn tokenize<'src>(input: &'src str) -> Tokenizer<'src> {
    Tokenizer {input: input}
}
