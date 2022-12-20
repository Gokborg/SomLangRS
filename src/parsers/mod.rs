pub mod decparser;
pub mod assignparser;
pub mod exprparser;
pub mod stmtparser;
pub mod bodyparser;
pub mod ifparser;
pub mod typeparser;

pub type PResult<T> = Result<T, ()>;