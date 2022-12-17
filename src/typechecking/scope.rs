use std::collections::HashMap;

use crate::{span::Span, errorcontext::Error};

use super::types::Type;

pub struct Scopes {
    scopes: Vec<Scope>,
}

impl Scopes {
    pub fn new() -> Self {
        Self {scopes: vec![Scope::new()]}
    }

    pub fn put(&mut self, name: &str, kind: Type, span: Span) {
        // self.scopes.last_mut().unwrap().variables[name] = Variable {kind, span};
    }
}

struct Scope {
    pub variables: HashMap<String, Variable>
}

impl Scope {
    fn new() -> Self {
        Self {variables: HashMap::new()}
    }
}

struct Variable {
    kind: Type,
    span: Span
}