# KDL Spec

This is the kinda-formal specification for KDL, including the intended data
model and the grammar.

## Introduction

KDL is a node-oriented document language. Its niche and purpose overlaps with
XML, and as do many of its semantics. You can use KDL both as a configuration
language, and a data exchange or storage format, if you so choose.

## Components

### Document

The toplevel concept of KDL is a Document. A Document is composed of one or more
[Nodes](#node), separated by newlines and whitespace, and eventually terminated by an EOF.

#### Example

The following is a document composed of two toplevel nodes:

```kdl
foo {
    bar
}
baz
```

### Node

Being a node-oriented language means that the real core component of any KDL
document is the "node". Every node must have a name, which is either a legal
[Identifier](#identifier), or a quoted [String](#string).

Following the name are one or more [Whitespace](#whitespace) components,
followed by zero or more whitespace-separated [Values](#value) or
[Properties](#property). Finally, a node is terminated by either a
[Newline](#newline), a [Children Block](#children-block), a semicolon (`;`) or
the end of the
file/stream (an `EOF`).

When present in the list of Properties and Values, plain Values (those not
attached to a Property), each "anonymous" value should be treated as a
Property whose key is its current index among _anonymous values_ in the same
node, starting from 0, as a string. Named properties do not count towarrds
this index.

That is, the following two nodes are semantically equivalent:

```kdl
foo 1 key="val" 2
foo "0"=1 "1"=2 key="val"
```

#### Example

```kdl
foo 1 key="val" 3 {
    bar
    baz
}
```

### Identifier

A bare Identifier is composed of any unicode codepoint other than [non-initial
characters](#non-inidital-characters), followed by any number of unicode
codepoints other than [non-identifier characters](#non-identifier-characters).
Identifiers are terminated by [Whitespace](#whitespace) or
[Newlines](#newline).

### Non-initial characters

The following characters cannot be the first character in a bare
[Identifier](#identifier):

* Any of "/\\{};[]=,"
* Any decimal digit (0-9)
* Any [non-identifier characters](#non-identifier-characters)

### Non-identifier characters

The following characters cannot be used anywhere in a bare [Identifier](#identifier):

* Any codepoint with hexadecimal value `0x20` or below.
* Any codepoint with hexadecimal value higher than `0x10FFF`.
* Any of "\\{};[]=,"

## Full Grammar

```
// FIXME: I don't... think this is quite right?
nodes := linespace* (node (newline nodes)? linespace*)?

// FIXME: This is missing the newline at the end? And is the single-line-comment thing correct?
node := identifier (node-space node-argument)* (node-space node-document)? single-line-comment?
node-argument := prop | value
node-children := '{' nodes '}'
node-space := ws* escline ws* | ws+

// FIXME: This needs adjustment to the new, unicode-friendly version
identifier := [a-zA-Z] [a-zA-Z0-9!$%&'*+\-./:<>?@\^_|~]* | string
prop := identifier '=' value
value := string | raw_string | number | boolean | 'null'

string := '"' character* '"'
character := '\' escape | [^\"]
escape := ["\\/bfnrt] | 'u{' hex-digit{1, 6} '}'
hex-digit := [0-9a-fA-F]

raw-string := 'r' raw-string-hash
raw-string-hash := '#' raw-string-hash '#' | raw-string-quotes
raw-string-quotes := '"' .* '"'

number := decimal | hex | octal | binary

decimal := integer ('.' [0-9]+)? exponent?
exponent := ('e' | 'E') integer
integer := sign? [0-9] [0-9_]*
sign := '+' | '-'

hex := '0x' hex-digit (hex-digit | '_')*
octal := '0o' [0-7] [0-7_]*
binary := '0b' ('0' | '1') ('0' | '1' | '_')*

boolean := 'true' | 'false'

escline := '\\' ws* (single-line-comment | newline)

linespace := newline | ws | single-line-comment

// FIXME: This needs to support all unicode newline chars. See #27
newline := ('\r' '\n') | '\n'

ws := bom | ' ' | '\t' | multi-line-comment | slashdash-comment

single-line-comment := '//' ('\r' [^\n] | [^\r\n])* newline
multi-line-comment := '/*' ('*' [^\/] | [^*])* '*/'
slashdash-comment := '/-' (node | value | prop | node-children)
```
