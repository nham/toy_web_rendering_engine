struct Parser {
    pos: uint,
    input: String,
}

impl Parser {
    // return the next character of unconsumed input
    fn next_char(&self) -> char {
        self.input.as_slice().char_at(self.pos)
    }
    
    // return if the unconsumed input starts with a str
    fn starts_with(&self, s: &str) -> bool {
        self.input.as_slice().slice_from(self.pos).starts_with(s)
    }

    // return whether all input is consumed
    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn consume_char(&mut self) -> char {
        let range = self.input.as_slice().char_range_at(self.pos);
        self.pos = range.next;
        range.ch
    }

    fn consume_while(&mut self, test: |char| -> bool) -> &str {
        let i = self.pos;
        while !self.eof() && test(self.next_char()) {
            self.consume_char();
        }

        self.input.as_slice().slice(i, self.pos)

    }

    fn consume_whitespace(&mut self) {
        self.consume_while(|c| c.is_whitespace());
    }

}
