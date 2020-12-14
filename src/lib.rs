use nom::combinator::all_consuming;
use nom::Err;

pub use crate::error::{KdlError, KdlErrorKind};
pub use crate::node::KdlNode;

mod error;
mod node;
mod parser;

pub fn parse_document<I>(input: I) -> Result<Vec<KdlNode>, KdlError>
where
    I: AsRef<str>,
{
    let input = &input.as_ref()[..];
    match all_consuming(parser::nodes)(input) {
        Ok((_, arg)) => Ok(arg),
        Err(err) => Err(match err {
            Err::Error(e) | Err::Failure(e) => KdlError {
                input: input.into(),
                offset: e.input.as_ptr() as usize - input.as_ptr() as usize,
                kind: if let Some(kind) = e.kind {
                    kind
                } else if let Some(ctx) = e.context {
                    KdlErrorKind::Context(ctx)
                } else {
                    KdlErrorKind::Other
                },
            },
            Err::Incomplete(_) => KdlError {
                input: input.into(),
                offset: input.len() - 1,
                kind: KdlErrorKind::IncompleteInput,
            },
        }),
    }
}
