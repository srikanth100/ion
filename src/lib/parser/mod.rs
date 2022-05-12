//! Take a Read instance and output statements
//!
//! The `Terminator` takes input data and creates string with the good size
//! The `StatementSplitter` than takes the data and produces statements, with the help of
//! `parse_and_validate`

/// The terminal tokens associated with the parsing process
pub mod lexers;
/// Parse the pipelines to a Pipeline struct
pub mod pipelines;
mod quotes;
mod statement;

pub use self::{
    quotes::Terminator,
    statement::{parse_and_validate, Error, StatementSplitter},
};

/// Fuzz method
#[cfg(fuzzing)]
pub mod fuzzing {
    use super::*;
    use crate::{
        builtins::BuiltinMap,
    };
    /// Fuzz parse
    pub fn statement_parse(data: &str) { statement::parse::parse(data, &BuiltinMap::new()); }
}
