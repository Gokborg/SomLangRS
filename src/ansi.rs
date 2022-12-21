use std::fmt::Display;

pub fn red<T>(value: T) -> Ansi<T> {
    Ansi { start: "31", value }
}
pub fn cyan<T>(value: T) -> Ansi<T> {
    Ansi { start: "36", value }
}
pub fn yellow<T>(value: T) -> Ansi<T> {
    Ansi { start: "33", value }
}

pub struct Ansi<T> {
    start: &'static str,
    value: T,
}

impl <T: Display> Display for Ansi<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[{}m{}\x1b[00m", self.start, self.value)
    }
}