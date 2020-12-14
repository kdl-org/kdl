use std::collections::HashMap;

use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take_until};
use nom::character::complete::{alpha1, alphanumeric1, char, one_of};
use nom::combinator::{eof, map, map_res, opt, recognize, value};
use nom::multi::{many0, many1};
use nom::sequence::{delimited, pair, preceded, terminated, tuple};
use nom::IResult;

use crate::error::KdlParseError;
use crate::node::{Node, NodeValue};

/// `nodes := linespace* (node (newline document)?)?`
pub(crate) fn nodes(input: &str) -> IResult<&str, Vec<Node>, KdlParseError<&str>> {
    many0(delimited(many0(linespace), node, newline))(input)
}

#[derive(Clone)]
enum NodeArg<'a> {
    Value(NodeValue),
    Property(&'a str, NodeValue),
}

/// `node := identifier (node-space node-argument)* (node-space node-document)?`
pub(crate) fn node(input: &str) -> IResult<&str, Node, KdlParseError<&str>> {
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
fn identifier(input: &str) -> IResult<&str, &str, KdlParseError<&str>> {
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

fn node_arg(input: &str) -> IResult<&str, NodeArg, KdlParseError<&str>> {
    alt((
        map(property, |(key, val)| NodeArg::Property(key, val)),
        map(node_value, NodeArg::Value),
    ))(input)
}

/// `prop := identifier '=' value`
fn property(input: &str) -> IResult<&str, (&str, NodeValue), KdlParseError<&str>> {
    let (input, key) = identifier(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, val) = node_value(input)?;
    Ok((input, (key, val)))
}

/// `value := string | raw_string | number | boolean | 'null'`
fn node_value(input: &str) -> IResult<&str, NodeValue, KdlParseError<&str>> {
    alt((
        map(string, |s| NodeValue::String(s.into())),
        map(raw_string, |s| NodeValue::String(s.into())),
        number,
        boolean,
        value(NodeValue::Null, tag("null")),
    ))(input)
}

/// `node-children := '{' nodes '}'`
fn node_children(input: &str) -> IResult<&str, Vec<Node>, KdlParseError<&str>> {
    delimited(tag("{"), nodes, tag("}"))(input)
}

// TODO: This should be much more specific about what escapes are allowed.
/// `string := '"' ('\\' ["\\] | [^"])* '"'`
fn string(_input: &str) -> IResult<&str, &str, KdlParseError<&str>> {
    todo!()
}

/// `raw-string := 'r' raw-string-hash`
/// `raw-string-hash := '#' raw-string-hash '#' | raw-string-quotes`
/// `raw-string-quotes := '"' .* '"'`
fn raw_string(input: &str) -> IResult<&str, &str, KdlParseError<&str>> {
    let (input, _) = char('r')(input)?;
    let (input, hashes) = recognize(many0(char('#')))(input)?;
    let (input, _) = char('"')(input)?;
    let close = format!("\"{}", hashes);
    let (input, string) = take_until(&close[..])(input)?;
    let (input, _) = tag(&close[..])(input)?;
    Ok((input, string))
}

/// `number := decimal | hex | octal | binary`
fn number(input: &str) -> IResult<&str, NodeValue, KdlParseError<&str>> {
    alt((
        map(integer, NodeValue::Int),
        map(hexadecimal, NodeValue::Int),
        map(octal, NodeValue::Int),
        map(binary, NodeValue::Int),
        map(float, NodeValue::Float),
    ))(input)
}

/// ```ignore
/// decimal := integer ('.' [0-9]+)? exponent?
/// exponent := ('e' | 'E') integer
/// integer := sign? [1-9] [0-9_]*
/// sign := '+' | '-'
/// ```
fn float(input: &str) -> IResult<&str, f64, KdlParseError<&str>> {
    map_res(
        alt((
            recognize(tuple((
                integer,
                opt(preceded(char('.'), integer)),
                one_of("eE"),
                opt(one_of("+-")),
                integer,
            ))),
            recognize(tuple((integer, char('.'), integer))),
        )),
        |x| str::replace(x, "_", "").parse::<f64>(),
    )(input)
}

/// ```ignore
/// decimal := integer ('.' [0-9]+)? exponent?
/// exponent := ('e' | 'E') integer
/// integer := sign? [1-9] [0-9_]*
/// sign := '+' | '-'
/// ```
fn integer(input: &str) -> IResult<&str, i64, KdlParseError<&str>> {
    let (input, sign) = opt(alt((char('+'), char('-'))))(input)?;
    let mult = if let Some(sign) = sign {
        if sign == '+' {
            1
        } else {
            -1
        }
    } else {
        1
    };
    map_res(
        recognize(many1(terminated(one_of("0123456789"), many0(char('_'))))),
        move |out: &str| {
            i64::from_str_radix(&str::replace(&out, "_", ""), 10).map(move |x| x * mult)
        },
    )(input)
}

/// `hex := '0x' [0-9a-fA-F] [0-9a-fA-F_]*`
fn hexadecimal(input: &str) -> IResult<&str, i64, KdlParseError<&str>> {
    map_res(
        preceded(
            alt((tag("0x"), tag("0X"))),
            recognize(many1(terminated(
                one_of("0123456789abcdefABCDEF"),
                many0(char('_')),
            ))),
        ),
        move |out: &str| i64::from_str_radix(&str::replace(&out, "_", ""), 16),
    )(input)
}

/// `octal := '0o' [0-7] [0-7_]*`
fn octal(input: &str) -> IResult<&str, i64, KdlParseError<&str>> {
    map_res(
        preceded(
            alt((tag("0o"), tag("0O"))),
            recognize(many1(terminated(one_of("01234567"), many0(char('_'))))),
        ),
        move |out: &str| i64::from_str_radix(&str::replace(&out, "_", ""), 8),
    )(input)
}

/// `binary := '0b' ('0' | '1') ('0' | '1' | '_')*`
fn binary(input: &str) -> IResult<&str, i64, KdlParseError<&str>> {
    map_res(
        preceded(
            alt((tag("0b"), tag("0B"))),
            recognize(many1(terminated(one_of("01"), many0(char('_'))))),
        ),
        move |out: &str| i64::from_str_radix(&str::replace(&out, "_", ""), 2),
    )(input)
}

/// `boolean := 'true' | 'false'`
fn boolean(input: &str) -> IResult<&str, NodeValue, KdlParseError<&str>> {
    alt((
        value(NodeValue::Boolean(true), tag("true")),
        value(NodeValue::Boolean(false), tag("false")),
    ))(input)
}

/// `node-space := ws* escline ws* | ws+`
fn node_space(input: &str) -> IResult<&str, (), KdlParseError<&str>> {
    alt((
        delimited(many0(whitespace), escline, many0(whitespace)),
        map(many1(whitespace), |_| ()),
    ))(input)
}

/// `single-line-comment := '//' ('\r' [^\n] | [^\r\n])* (newline | eof)`
fn single_line_comment(input: &str) -> IResult<&str, (), KdlParseError<&str>> {
    let (input, _) = tag("//")(input)?;
    let (input, _) = alt((take_until("\r\n"), is_not("\n")))(input)?;
    let (input, _) = alt((newline, value((), eof)))(input)?;
    Ok((input, ()))
}

/// `multi-line-comment := '/*' ('*' [^\/] | [^*])* '*/'`
fn multi_line_comment(input: &str) -> IResult<&str, (), KdlParseError<&str>> {
    delimited(tag("/*"), value((), take_until("*/")), tag("*/"))(input)
}

/// `escline := '\\' ws* (single-line-comment | newline)`
fn escline(input: &str) -> IResult<&str, (), KdlParseError<&str>> {
    let (input, _) = tag("\\")(input)?;
    let (input, _) = many0(whitespace)(input)?;
    let (input, _) = alt((single_line_comment, newline))(input)?;
    Ok((input, ()))
}

/// `linespace := newline | ws | single-line-comment`
fn linespace(input: &str) -> IResult<&str, (), KdlParseError<&str>> {
    value((), alt((newline, whitespace, single_line_comment)))(input)
}

/// `ws := bom | ' ' | '\t' | multi-line-comment`
fn whitespace(input: &str) -> IResult<&str, (), KdlParseError<&str>> {
    // TODO: bom?
    value(
        (),
        alt((
            /*bom,*/ tag(" "),
            tag("\t"),
            recognize(multi_line_comment),
        )),
    )(input)
}

/// `newline := ('\r' '\n') | '\n'`
fn newline(input: &str) -> IResult<&str, (), KdlParseError<&str>> {
    value((), alt((tag("\r\n"), tag("\n"))))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_float() {
        assert_eq!(float("1.0"), Ok(("", 1.0f64)));
        assert_eq!(float("0.0"), Ok(("", 0.0f64)));
        assert_eq!(float("-1.0"), Ok(("", -1.0f64)));
        assert_eq!(float("+1.0"), Ok(("", 1.0f64)));
        assert_eq!(float("1.0e10"), Ok(("", 1.0e10f64)));
        assert_eq!(float("1.0e-10"), Ok(("", 1.0e-10f64)));
        assert_eq!(float("-1.0e-10"), Ok(("", -1.0e-10f64)));
        assert_eq!(float("123_456_789.0"), Ok(("", 123456789.0f64)));
        assert_eq!(float("123_456_789.0_"), Ok(("", 123456789.0f64)));
        assert!(float("?1.0").is_err());
        assert!(float("_1.0").is_err());
        assert!(float("1._0").is_err());
        assert!(float("1.").is_err());
        assert!(float(".0").is_err());
    }

    #[test]
    fn test_integer() {
        assert_eq!(integer("0123456789"), Ok(("", 123456789)));
        assert_eq!(integer("0123_456_789"), Ok(("", 123456789)));
        assert_eq!(integer("0123_456_789_"), Ok(("", 123456789)));
        assert_eq!(integer("+0123456789"), Ok(("", 123456789)));
        assert_eq!(integer("-0123456789"), Ok(("", -123456789)));
        assert!(integer("?0123456789").is_err());
        assert!(integer("_0123456789").is_err());
        assert!(integer("a").is_err());
        assert!(integer("--").is_err());
    }

    #[test]
    fn test_hexadecimal() {
        assert_eq!(
            hexadecimal("0x0123456789abcdef"),
            Ok(("", 0x0123456789abcdef))
        );
        assert_eq!(
            hexadecimal("0x01234567_89abcdef"),
            Ok(("", 0x0123456789abcdef))
        );
        assert_eq!(
            hexadecimal("0x01234567_89abcdef_"),
            Ok(("", 0x0123456789abcdef))
        );
        assert!(hexadecimal("0x_123").is_err());
        assert!(hexadecimal("0xg").is_err());
        assert!(hexadecimal("0xx").is_err());
    }

    #[test]
    fn test_octal() {
        assert_eq!(octal("0o01234567"), Ok(("", 0o01234567)));
        assert_eq!(octal("0o0123_4567"), Ok(("", 0o01234567)));
        assert_eq!(octal("0o01234567_"), Ok(("", 0o01234567)));
        assert!(octal("0o_123").is_err());
        assert!(octal("0o8").is_err());
        assert!(octal("0oo").is_err());
    }

    #[test]
    fn test_binary() {
        assert_eq!(binary("0b0101"), Ok(("", 0b0101)));
        assert_eq!(binary("0b01_10"), Ok(("", 0b0110)));
        assert_eq!(binary("0b01___10"), Ok(("", 0b0110)));
        assert_eq!(binary("0b0110_"), Ok(("", 0b0110)));
        assert!(binary("0b_0110").is_err());
        assert!(binary("0b20").is_err());
        assert!(binary("0bb").is_err());
    }

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
        assert_eq!(whitespace(" "), Ok(("", ())));
        assert_eq!(whitespace("\t"), Ok(("", ())));
        assert_eq!(whitespace("/* \nfoo\r\n */ etc"), Ok((" etc", ())));
        assert!(whitespace("hi").is_err())
    }

    #[test]
    fn test_newline() {
        assert_eq!(newline("\n"), Ok(("", ())));
        assert_eq!(newline("\r\n"), Ok(("", ())));
        assert_eq!(newline("\n\n"), Ok(("\n", ())));
        assert!(newline("\r").is_err());
        assert!(newline("blah").is_err());
    }
}
