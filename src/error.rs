// src/error.rs
//! Error types for user-friendly ID operations

use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum IdError {
    #[error("Invalid character in ID")]
    InvalidCharacter,
    #[error("Invalid check bit")]
    InvalidCheckBit,
    #[error("ID length too short, minimum 3 characters")]
    TooShort,
    #[error("ID length exceeds maximum allowed length")]
    TooLong,
    #[error("Invalid sequence in ID")]
    InvalidSequence,
}
