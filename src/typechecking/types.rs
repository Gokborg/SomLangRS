pub enum Type {
    Int,
    Bool,
    Pointer(Box<Type>),
    Array(Box<Type>),
}