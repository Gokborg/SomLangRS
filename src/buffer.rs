#[derive(Default)]
pub struct Buffer {
    content: Vec<char>,
    pub pos: usize,
}

impl Buffer {
    fn new(content: Vec<char>) -> Self {
        Self { content, pos: 0 }
    }

    fn done(&self) -> bool {
        return self.pos >= self.content.len();
    }
    
    fn next(&mut self) -> Option<char> {
        if self.pos >= self.content.len() {
            return None;
        }
        self.pos += 1;
        return Some(self.content[self.pos - 1]);
    }
    
    fn peek(&mut self) -> Option<&char> {
        return self.content.get(self.pos);
    }

    fn next_if<F: Fn(&char) -> bool>(&mut self, f: F) -> Option<char> {
        let value = self.peek();
        if let Some(value) = value {
            if !f(value) {
                return None;
            }
            self.pos += 1;
            return Some(*value);
        }
        return None;
    }

    fn next_while<F: Fn(char) -> bool>(&mut self, f: F) {
        let conn: String = self.content.get(self.pos).to_owned();
        while self.next_if(f).is_some() {
            conn.push(self.content[self.pos]);
        }
    }
}