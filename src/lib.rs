use nom::combinator::all_consuming;
use nom::Err;

pub use crate::error::{Error, ErrorKind};
pub use crate::node::Node;

mod error;
mod node;
mod parser;

pub fn parse_document<I>(input: I) -> Result<Vec<Node>, Error>
where
    I: AsRef<str>,
{
    let input = &input.as_ref()[..];
    match all_consuming(parser::nodes)(input) {
        Ok((_, arg)) => Ok(arg),
        Err(err) => Err(match err {
            Err::Error(e) | Err::Failure(e) => Error {
                input: input.into(),
                offset: e.input.as_ptr() as usize - input.as_ptr() as usize,
                kind: if let Some(kind) = e.kind {
                    kind
                } else if let Some(ctx) = e.context {
                    ErrorKind::Context(ctx)
                } else {
                    ErrorKind::Other
                },
            },
            Err::Incomplete(_) => Error {
                input: input.into(),
                offset: input.len() - 1,
                kind: ErrorKind::IncompleteInput,
            },
        }),
    }
}
