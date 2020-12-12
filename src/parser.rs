use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::multi::{many0, many1};
use nom::sequence::{delimited, preceded};
use nom::IResult;

use crate::error::{ErrorKind, NodeParseError};
use crate::node::Node;

/// `document := linespace* (node (newline document)?)?`
pub(crate) fn nodes(input: &str) -> IResult<&str, Vec<Node>, NodeParseError<&str>> {
    // TODO: this is wrong
    many0(node)(input)
}

/// `node := identifier (node-space node-argument)* (node-space node-document)?`
pub(crate) fn node(input: &str) -> IResult<&str, Node, NodeParseError<&str>> {
    let (input, tag) = identifier(input)?;
    let (input, args) = many0(preceded(node_space, node_arg))(input)?;
    let (input, children) = opt(preceded(node_space, node_children))(input)?;
    todo!();
}

/// `identifier := [a-zA-Z] [a-zA-Z0-9!#$%&'*+\-./:<>?@\^_|~]* | string`
fn identifier(input: &str) -> IResult<&str, &str, NodeParseError<&str>> {
    todo!()
}

/// `node-space := ws* escline ws* | ws+`
fn node_space(input: &str) -> IResult<&str, (), NodeParseError<&str>> {
    alt((
        delimited(many0(whitespace), escline, many0(whitespace)),
        map(many1(whitespace), |_| ()),
    ))(input)
}

/// `single-line-comment := '//' ('\r' [^\n] | [^\r\n])* newline`
fn single_line_comment(input: &str) -> IResult<&str, &str, NodeParseError<&str>> {
    todo!()
}

/// `multi-line-comment := '/*' ('*' [^\/] | [^*])* '*/'`
fn multi_line_comment(input: &str) -> IResult<&str, &str, NodeParseError<&str>> {
    todo!()
}

/// `escline := '\\' ws* (single-line-comment | newline)`
fn escline(input: &str) -> IResult<&str, (), NodeParseError<&str>> {
    let (input, _) = tag("\\")(input)?;
    let (input, _) = many0(whitespace)(input)?;
    let (input, _) = alt((single_line_comment, newline))(input)?;
    Ok((input, ()))
}

/// `ws := bom | ' ' | '\t' | multi-line-comment`
fn whitespace(input: &str) -> IResult<&str, &str, NodeParseError<&str>> {
    // TODO: bom?
    alt((/*bom,*/ tag(" "), tag("\t"), multi_line_comment))(input)
}

/// `newline := ('\r' '\n') | '\n'`
fn newline(input: &str) -> IResult<&str, &str, NodeParseError<&str>> {
    alt((tag("\r\n"), tag("\n")))(input)
}
