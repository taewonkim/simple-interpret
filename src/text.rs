pub struct Line {
    pub text: String,
    pub pos: usize,
}

impl Line {
    pub fn new(s: String) -> Line {
        Line { text: s, pos: 0 }
    }

    pub fn next(&mut self) -> &str {
        let tmp = self.pos;
        self.pos += 1;
        &self.text[tmp..(tmp+1)]
    }
}
