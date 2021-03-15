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

    pub fn expr(&mut self) -> i32 {
        let left = (self.next().to_string()).parse::<i32>().unwrap();
        let op = self.next().to_string();
        let right = (self.next().to_string()).parse::<i32>().unwrap();

        if op == "+" {
            left + right
        }
        else
        if op == "-" {
            left - right
        }
        else
        if op == "*" {
            left * right
        }
        else
        if op == "/" {
            left / right
        }
        else
        if op == "%" {
            left % right
        }
        else {
            0
        }
    }
}

