use std::collections::HashMap;

use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take_until};
use nom::character::complete::{alpha1, alphanumeric1, char, one_of};
use nom::combinator::{eof, map, opt, recognize, value};
use nom::multi::{many0, many1};
use nom::sequence::{delimited, pair, preceded};
use nom::IResult;

use crate::error::NodeParseError;
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
    alt((
        map(property, |(key, val)| NodeArg::Property(key, val)),
        map(node_value, NodeArg::Value),
    ))(input)
}

/// `prop := identifier '=' value`
fn property(input: &str) -> IResult<&str, (&str, NodeValue), NodeParseError<&str>> {
    let (input, key) = identifier(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, val) = node_value(input)?;
    Ok((input, (key, val)))
}

/// `value := string | raw_string | number | boolean | 'null'`
fn node_value(input: &str) -> IResult<&str, NodeValue, NodeParseError<&str>> {
    alt((
        map(string, |s| NodeValue::String(s.into())),
        map(raw_string, |s| NodeValue::String(s.into())),
        number,
        boolean,
        value(NodeValue::Null, tag("null")),
    ))(input)
}

fn node_children(_input: &str) -> IResult<&str, Vec<Node>, NodeParseError<&str>> {
    todo!()
}

// TODO: This should be much more specific about what escapes are allowed.
/// `string := '"' ('\\' ["\\] | [^"])* '"'`
fn string(_input: &str) -> IResult<&str, &str, NodeParseError<&str>> {
    todo!()
}

// TODO: this is clever but... I don't like the recursion here.
/// `raw-string := 'r' raw-string-hash`
/// `raw-string-hash := '#' raw-string-hash '#' | raw-string-quotes`
/// `raw-string-quotes := '"' .* '"'`
fn raw_string(input: &str) -> IResult<&str, &str, NodeParseError<&str>> {
    let (input, _) = char('r')(input)?;
    let (input, hashes) = recognize(many0(char('#')))(input)?;
    let (input, _) = char('"')(input)?;
    let close = format!("\"{}", hashes);
    let (input, string) = take_until(&close[..])(input)?;
    let (input, _) = tag(&close[..])(input)?;
    Ok((input, string))
}

/// `number := decimal | hex | octal | binary`
fn number(_input: &str) -> IResult<&str, NodeValue, NodeParseError<&str>> {
    todo!()
}

/// `boolean := 'true' | 'false'`
fn boolean(input: &str) -> IResult<&str, NodeValue, NodeParseError<&str>> {
    alt((
        value(NodeValue::Boolean(true), tag("true")),
        value(NodeValue::Boolean(false), tag("false")),
    ))(input)
}

/// `node-space := ws* escline ws* | ws+`
fn node_space(input: &str) -> IResult<&str, (), NodeParseError<&str>> {
    alt((
        delimited(many0(whitespace), escline, many0(whitespace)),
        map(many1(whitespace), |_| ()),
    ))(input)
}

/// `single-line-comment := '//' ('\r' [^\n] | [^\r\n])* (newline | eof)`
fn single_line_comment(input: &str) -> IResult<&str, (), NodeParseError<&str>> {
    let (input, _) = tag("//")(input)?;
    let (input, _) = alt((take_until("\r\n"), is_not("\n")))(input)?;
    let (input, _) = alt((newline, eof))(input)?;
    Ok((input, ()))
}

/// `multi-line-comment := '/*' ('*' [^\/] | [^*])* '*/'`
fn multi_line_comment(input: &str) -> IResult<&str, (), NodeParseError<&str>> {
    delimited(tag("/*"), value((), take_until("*/")), tag("*/"))(input)
}

/// `escline := '\\' ws* (single-line-comment | newline)`
fn escline(input: &str) -> IResult<&str, (), NodeParseError<&str>> {
    let (input, _) = tag("\\")(input)?;
    let (input, _) = many0(whitespace)(input)?;
    let (input, _) = alt((recognize(single_line_comment), newline))(input)?;
    Ok((input, ()))
}

/// `ws := bom | ' ' | '\t' | multi-line-comment`
fn whitespace(input: &str) -> IResult<&str, &str, NodeParseError<&str>> {
    // TODO: bom?
    alt((
        /*bom,*/ tag(" "),
        tag("\t"),
        recognize(multi_line_comment),
    ))(input)
}

/// `newline := ('\r' '\n') | '\n'`
fn newline(input: &str) -> IResult<&str, &str, NodeParseError<&str>> {
    alt((tag("\r\n"), tag("\n")))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raw_string() {
        assert_eq!(raw_string(r#"r"foo""#), Ok(("", "foo")));
        assert_eq!(raw_string("r\"foo\nbar\""), Ok(("", "foo\nbar")));
        assert_eq!(raw_string(r##"r#"foo"#"##), Ok(("", "foo")));
        assert_eq!(raw_string(r###"r##"foo"##"###), Ok(("", "foo")));
        assert_eq!(raw_string(r#"r"\nfoo\r""#), Ok(("", r"\nfoo\r")));
        assert!(raw_string(r###"r##"foo"#"###).is_err());
    }

    #[test]
    fn test_boolean() {
        assert_eq!(boolean("true"), Ok(("", NodeValue::Boolean(true))));
        assert_eq!(boolean("false"), Ok(("", NodeValue::Boolean(false))));
        assert!(boolean("blah").is_err());
    }

    #[test]
    fn test_node_space() {
        assert_eq!(node_space(" "), Ok(("", ())));
        assert_eq!(node_space("\t "), Ok(("", ())));
        assert_eq!(node_space("\t \\ // hello\n "), Ok(("", ())));
        assert!(node_space("blah").is_err());
    }

    #[test]
    fn test_single_line_comment() {
        assert_eq!(single_line_comment("//hello"), Ok(("", ())));
        assert_eq!(single_line_comment("// \thello"), Ok(("", ())));
        assert_eq!(single_line_comment("//hello\n"), Ok(("", ())));
        assert_eq!(single_line_comment("//hello\r\n"), Ok(("", ())));
        assert_eq!(single_line_comment("//hello\n\r"), Ok(("\r", ())));
        assert_eq!(single_line_comment("//hello\rworld"), Ok(("", ())));
    }

    #[test]
    fn test_multi_line_comment() {
        assert_eq!(multi_line_comment("/*hello*/"), Ok(("", ())));
        assert_eq!(multi_line_comment("/*hello*/\n"), Ok(("\n", ())));
        assert_eq!(multi_line_comment("/*\nhello\r\n*/"), Ok(("", ())));
        assert_eq!(multi_line_comment("/*\nhello** /\n*/"), Ok(("", ())));
        assert_eq!(multi_line_comment("/**\nhello** /\n*/"), Ok(("", ())));
        assert_eq!(multi_line_comment("/*hello*/world"), Ok(("world", ())));
    }

    #[test]
    fn test_escline() {
        assert_eq!(escline("\\\nfoo"), Ok(("foo", ())));
        assert_eq!(escline("\\\n  foo"), Ok(("  foo", ())));
        assert_eq!(escline("\\  \t \nfoo"), Ok(("foo", ())));
        assert_eq!(escline("\\ // test \nfoo"), Ok(("foo", ())));
        assert_eq!(escline("\\ // test \n  foo"), Ok(("  foo", ())));
    }

    #[test]
    fn test_whitespace() {
        assert_eq!(whitespace(" "), Ok(("", " ")));
        assert_eq!(whitespace("\t"), Ok(("", "\t")));
        assert_eq!(
            whitespace("/* \nfoo\r\n */ etc"),
            Ok((" etc", "/* \nfoo\r\n */"))
        );
        assert!(whitespace("hi").is_err())
    }

    #[test]
    fn test_newline() {
        assert_eq!(newline("\n"), Ok(("", "\n")));
        assert_eq!(newline("\r\n"), Ok(("", "\r\n")));
        assert_eq!(newline("\n\n"), Ok(("\n", "\n")));
        assert!(newline("\r").is_err());
        assert!(newline("blah").is_err());
    }
}
