use std::collections::HashMap;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1, one_of};
use nom::combinator::{map, not, opt, recognize};
use nom::multi::{many0, many1};
use nom::sequence::{delimited, pair, preceded};
use nom::IResult;

use crate::error::{ErrorKind, NodeParseError};
use crate::node::{Node, NodeValue};

/// `document := linespace* (node (newline document)?)?`
pub(crate) fn nodes(input: &str) -> IResult<&str, Vec<Node>, NodeParseError<&str>> {
    // TODO: this is wrong
    many0(node)(input)
}

#[derive(Clone)]
enum NodeArg<'a> {
    Value(NodeValue),
    Property(&'a str, NodeValue),
}

/// `node := identifier (node-space node-argument)* (node-space node-document)?`
pub(crate) fn node(input: &str) -> IResult<&str, Node, NodeParseError<&str>> {
    let (input, tag) = identifier(input)?;
    let (input, args) = many0(preceded(node_space, node_arg))(input)?;
    let (input, children) = opt(preceded(node_space, node_children))(input)?;
    let (values, properties): (Vec<NodeArg>, Vec<NodeArg>) = args
        .into_iter()
        .partition(|arg| matches!(arg, NodeArg::Value(_)));
    Ok((
        input,
        Node {
            name: tag.into(),
            children: children.unwrap_or_else(Vec::new),
            values: values
                .into_iter()
                .map(|arg| match arg {
                    NodeArg::Value(val) => val,
                    _ => unreachable!(),
                })
                .collect(),
            properties: properties.into_iter().fold(HashMap::new(), |mut acc, arg| {
                match arg {
                    NodeArg::Property(key, value) => {
                        acc.insert(String::from(key), value);
                    }
                    _ => unreachable!(),
                }
                acc
            }),
        },
    ))
}

/// `identifier := [a-zA-Z_] [a-zA-Z0-9!#$%&'*+\-./:<>?@\^_|~]* | string`
fn identifier(input: &str) -> IResult<&str, &str, NodeParseError<&str>> {
    alt((
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((
                alphanumeric1,
                recognize(one_of("~!@#$%^&*-_+./:<>?")),
            ))),
        )),
        string,
    ))(input)
}

fn node_arg(input: &str) -> IResult<&str, NodeArg, NodeParseError<&str>> {
    todo!()
}

fn node_children(input: &str) -> IResult<&str, Vec<Node>, NodeParseError<&str>> {
    todo!()
}

// TODO: This should be much more specific about what escapes are allowed.
/// `string := '"' ('\\' ["\\] | [^"])* '"'`
fn string(input: &str) -> IResult<&str, &str, NodeParseError<&str>> {
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
    let (input, _) = tag("//")(input)?;
    let (input, comment) = recognize(many0(alt((
        preceded(tag("\r"), not(tag("\n"))),
        not(tag("\r\n")),
    ))))(input)?;
    let (input, _) = newline(input)?;
    Ok((input, comment))
}

/// `multi-line-comment := '/*' ('*' [^\/] | [^*])* '*/'`
fn multi_line_comment(input: &str) -> IResult<&str, &str, NodeParseError<&str>> {
    delimited(tag("/*"), recognize(many0(not(tag("*/")))), tag("*/"))(input)
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
