use nom::error::{ContextError, ParseError};

use thiserror::Error;
#[derive(Debug, Clone, Eq, PartialEq, Error)]
#[error("Error parsing document. {kind}")]
pub struct Error {
    pub input: String,
    pub offset: usize,
    pub kind: ErrorKind,
}

#[derive(Debug, Clone, Eq, PartialEq, Error)]
pub enum ErrorKind {
    #[error("Failed to parse {0} component of semver string.")]
    Context(&'static str),
    #[error("Incomplete input to semver parser.")]
    IncompleteInput,
    #[error("An unspecified error occurred.")]
    Other,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct NodeParseError<I> {
    pub(crate) input: I,
    pub(crate) context: Option<&'static str>,
    pub(crate) kind: Option<ErrorKind>,
}

impl<I> ParseError<I> for NodeParseError<I> {
    fn from_error_kind(input: I, _kind: nom::error::ErrorKind) -> Self {
        Self {
            input,
            context: None,
            kind: None,
        }
    }

    fn append(_input: I, _kind: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}

impl<I> ContextError<I> for NodeParseError<I> {
    fn add_context(_input: I, ctx: &'static str, mut other: Self) -> Self {
        other.context = Some(ctx);
        other
    }
}
