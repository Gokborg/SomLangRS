#[derive(Default)]
pub struct Buffer<T> {
    pos: usize,
    current: T,
    content: Vec<T>,
    done: bool
}

impl<T> Buffer <T> {
    fn set(&mut self, content: Vec<T>) {
        self.content = content;
        self.current = content[0];
        self.pos = 0;
        self.done = false;
    }

    fn next(&mut self) -> T {
        self.pos += 1;
        if self.pos < self.content.len() {
            self.current = self.current[self.pos];
        }
        else {
            self.done = true;
            self.current = '\0';
        }
        return self.current
    }
}