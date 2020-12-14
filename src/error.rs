use std::num::{ParseFloatError, ParseIntError};

use nom::error::{ContextError, ErrorKind, FromExternalError, ParseError};

use thiserror::Error;
#[derive(Debug, Clone, Eq, PartialEq, Error)]
#[error("Error parsing document. {kind}")]
pub struct KdlError {
    pub input: String,
    pub offset: usize,
    pub kind: KdlErrorKind,
}

#[derive(Debug, Clone, Eq, PartialEq, Error)]
pub enum KdlErrorKind {
    #[error(transparent)]
    ParseIntError(ParseIntError),
    #[error(transparent)]
    ParseFloatError(ParseFloatError),
    #[error("Failed to parse {0} component of semver string.")]
    Context(&'static str),
    #[error("Incomplete input to semver parser.")]
    IncompleteInput,
    #[error("An unspecified error occurred.")]
    Other,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct KdlParseError<I> {
    pub(crate) input: I,
    pub(crate) context: Option<&'static str>,
    pub(crate) kind: Option<KdlErrorKind>,
}

impl<I> ParseError<I> for KdlParseError<I> {
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

impl<I> ContextError<I> for KdlParseError<I> {
    fn add_context(_input: I, ctx: &'static str, mut other: Self) -> Self {
        other.context = Some(ctx);
        other
    }
}

impl<'a> FromExternalError<&'a str, ParseIntError> for KdlParseError<&'a str> {
    fn from_external_error(input: &'a str, _kind: ErrorKind, e: ParseIntError) -> Self {
        KdlParseError {
            input,
            context: None,
            kind: Some(KdlErrorKind::ParseIntError(e)),
        }
    }
}

impl<'a> FromExternalError<&'a str, ParseFloatError> for KdlParseError<&'a str> {
    fn from_external_error(input: &'a str, _kind: ErrorKind, e: ParseFloatError) -> Self {
        KdlParseError {
            input,
            context: None,
            kind: Some(KdlErrorKind::ParseFloatError(e)),
        }
    }
}
