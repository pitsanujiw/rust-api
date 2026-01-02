use std::fmt;

#[derive(Debug)]
pub enum DomainError {
    NotFound,
    Conflict(String),
    Validation(String),
    Unexpected(String),
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DomainError::NotFound => write!(f, "not found"),
            DomainError::Conflict(s) => write!(f, "conflict: {s}"),
            DomainError::Validation(s) => write!(f, "validation: {s}"),
            DomainError::Unexpected(s) => write!(f, "unexpected: {s}"),
        }
    }
}

impl std::error::Error for DomainError {}
