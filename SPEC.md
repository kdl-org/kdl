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

Following the name are zero or more [Values](#value) or
[Properties](#property), separated by either [whitespace](#whitespace) or [a
slash-escaped line continuation](#line-continuation). Values and Properties
may be interspersed in any order, much like is common with positional
arguments vs options in command line tools.

Values are ordered relative to each other and that order must be
preserved in order to maintain the semantics.

By contrast, Property order _should not matter_ to implementations.
[Children](#children-block) should be used if an order-sensitive key/value
data structure must be represented in KDL.

Finally, a node is terminated by either a [Newline](#newline), a [Children
Block](#children-block), a semicolon (`;`) or the end of the file/stream (an
`EOF`).

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

The following characters cannot be used anywhere in a bare
[Identifier](#identifier):

* Any codepoint with hexadecimal value `0x20` or below.
* Any codepoint with hexadecimal value higher than `0x10FFF`.
* Any of "\\{};[]=,"

### Line Continuation

Line continuations allow [Nodes](#node) to be spread across multiple lines.

A line continuation is one or more [whitespace](#whitespace) characters,
followed by a `/` character. This character can then be followed by more
[whitespace](#whitespace) and must be terminated by a [Newline](#newline)
(including the Newline that is part of single-line comments).

Following a line continuation, processing of a Node can continue as usual.

#### Example
```kdl
my-node 1 2 \  // this is a comment
        3 4    // This is the actual end of the Node.
```

### Newline

The following characters [should be treated as new
lines](https://www.unicode.org/versions/Unicode13.0.0/ch05.pdf):

| Acronym | Name            | Unicode |
|---------|-----------------|---------|
| CR      | Carriage Return | `000D`  |
| LF      | Line Feed       | `000A`  |
| CRLF    | Carriage Return and Line Feed | `000D` + `000A` |
| NEL     | Next Line       | `0085`  |
| VT      | Vertical Tab    | `000B`  |
| FF      | Form Feed       | `000C`  |
| LS      | Line Separator  | `2028`  |
| PS      | Paragraph Separator | `2029` |

Note that for the purpose of new lines, CRLF is considered _a single newline_.

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

newline := `000D` | `000A` | `000D` `000A` | `0085` | `000B` | `000C` | `2028` | `2029`

ws := bom | ' ' | '\t' | multi-line-comment | slashdash-comment

single-line-comment := '//' ('\r' [^\n] | [^\r\n])* newline
multi-line-comment := '/*' ('*' [^\/] | [^*])* '*/'
slashdash-comment := '/-' (node | value | prop | node-children)
```
