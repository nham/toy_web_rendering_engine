use dom;

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

    fn consume_ascii_alphanumeric(&mut self) -> &str {
        self.consume_while(|c| match c {
            'a'..'z' | 'A'..'Z' | '0'..'9' => true,
            _ => false,
        })
    }

    fn consume_printable_ascii_no_dash(&mut self) -> &str {
        self.consume_while(|c| c >= ' ' && c <= '~' && c != '-');
    }

    fn parse_node(&mut self) -> dom::Node {
        match self.next_char() {
            '<' =>
                if self.starts_with("<!--") {
                    self.parse_comment()
                } else {
                    self.parse_element()
                },
            _ => self.parse_text(),
        }
    }

    fn parse_text(&mut self) -> dom::Node {
        let s = self.consume_while(|c| c != '<');
        dom::Node::text(String::from_str(s))
    }

    fn parse_comment(&mut self) -> dom::Node {
        assert!(self.consume_char() == '<');
        assert!(self.consume_char() == '!');
        assert!(self.consume_char() == '-');
        assert!(self.consume_char() == '-');
        // TODO: consume dashes as long as its not end of comment
        let text = self.consume_printable_ascii_no_dash();
        assert!(self.consume_char() == '-');
        assert!(self.consume_char() == '-');
        assert!(self.consume_char() == '>');
        dom::Node::comment(String::from_str(text))
    }


    // straight up stolen from mbrubeck's post
    fn parse_element(&mut self) -> dom::Node {
        // Opening tag.
        assert!(self.consume_char() == '<');
        let tag_name = self.consume_ascii_alphanumeric();
        let attrs = self.parse_attributes();
        assert!(self.consume_char() == '>');

        // Contents.
        let children = self.parse_nodes();

        // Closing tag.
        assert!(self.consume_char() == '<');
        assert!(self.consume_char() == '/');
        assert!(self.consume_ascii_alphanumeric() == tag_name);
        assert!(self.consume_char() == '>');

        return dom::Node::elem(String::from_str(tag_name), attrs, children);
    }

}
