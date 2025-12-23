use std::io;
use thiserror::Error;

// Implement custom error type
#[derive(Error, Debug)]
pub enum Error{
    #[error("IO error occurred: {0}")]
    Io(#[from] io::Error),
    #[error("Insufficient operands for operation")]
    InsufficientOperands,
    #[error("Division by zero")]
    DivisionByZero,
    #[error("Mismatched parentheses")]
    MismatchedParentheses,
    #[error("Unacceptable token encountered")]
    UnacceptableToken,
}

pub type Result<T> = std::result::Result<T, Error>;