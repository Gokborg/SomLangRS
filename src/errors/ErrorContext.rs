pub struct ErrorContext {
    errors: Vec<Error>
}

impl ErrorContext {
    pub fn new() -> Self {
        Self {errors: Vec::new()}
    }
}


pub struct ErrorKind {
    
}

pub struct Error {
    span: Span,
}