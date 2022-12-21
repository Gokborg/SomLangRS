use std::{collections::HashMap, ops::{Index, IndexMut}};

use crate::{span::Span, errorcontext::Error};

use super::{att::Type};

#[derive(Debug)]
pub struct Variable {
    pub vartype: Type,
    declaration: Span,
    references: Vec<Span>,
}

#[derive(Debug, Clone, Copy)]
pub struct VarIndex {
    index: usize
}

#[derive(Debug)]
pub struct Scopes {
    scopes: Vec<Scope>,
    variables: Vec<Variable>,
}

impl Scopes {
    pub fn new() -> Self {
        let mut top = Scope::new();
        top.types.insert("uint".to_owned(), Type::UInt);
        top.types.insert("bool".to_owned(), Type::Bool);
        top.types.insert("char".to_owned(), Type::Char);

        Self {scopes: vec![top], variables: Vec::new()}
    }

    pub fn push(&mut self) {
        self.scopes.push(Scope::new());
    }
    pub fn pop(&mut self) {
        if self.scopes.len() < 2 {
            panic!("scope underflow");
        }
        self.scopes.pop();
    }

    pub fn get_type(&self, name: &str) -> Option<Type> {
        for scope in self.scopes.iter().rev() {
            if let Some(vartype) = scope.types.get(name) {
                return Some(vartype.clone());
            }
        }
        None
    }

    pub fn put(&mut self, name: String, vartype: Type, span: Span) -> VarIndex  {
        let var_index = VarIndex {index: self.variables.len()};
        self.variables.push(Variable {vartype, declaration: span, references: Vec::new()});
        let last_index = self.scopes.len() - 1;
        self.scopes[last_index].variables.insert(name, var_index);
        return var_index;
    }
    pub fn get_index(&self, name: &str) -> Option<VarIndex> {
        for scope in self.scopes.iter().rev() {
            if let Some(&index) = scope.variables.get(name) {
                return Some(index);
            }
        }
        None
    }
    pub fn get(&self, name: &str) -> Option<&Variable> {
        self.get_index(name).map(|i| &self[i])
    }
}

impl IndexMut<VarIndex> for Scopes {
    fn index_mut(&mut self, index: VarIndex) -> &mut Self::Output {
        self.variables.index_mut(index.index)
    }
}

impl Index<VarIndex> for Scopes {
    type Output = Variable;

    fn index(&self, index: VarIndex) -> &Self::Output {
        self.variables.index(index.index)
    }
}

#[derive(Debug)]
struct Scope {
    pub variables: HashMap<String, VarIndex>,
    pub types: HashMap<String, Type>,
}

impl Scope {
    fn new() -> Self {
        Self {variables: HashMap::new(), types: HashMap::new()}
    }
}