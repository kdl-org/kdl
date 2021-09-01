# KDL Spec

This is the semi-formal specification for KDL, including the intended data
model and the grammar.

This document describes KDL version `1.0.0-pre.0`.

## Introduction

KDL is a node-oriented document language. Its niche and purpose overlaps with
XML, and as do many of its semantics. You can use KDL both as a configuration
language, and a data exchange or storage format, if you so choose.

The bulk of this document is dedicated to a long-form description of all
[Components](#components) of a KDL documeent. There is also a much more terse
[Grammar](#full-grammar) at the end of the document that covers most of the
rules, with some semantic exceptions involving the data model.

KDL is designed to be easy to read _and_ easy to implement.

## Components

### Document

The toplevel concept of KDL is a Document. A Document is composed of zero or
more [Nodes](#node), separated by newlines and whitespace, and eventually
terminated by an EOF.

All KDL documents should be UTF-8 encoded and conform to the specifications in
this document.

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

Following the name are zero or more [Arguments](#argument) or
[Properties](#property), separated by either [whitespace](#whitespace) or [a
slash-escaped line continuation](#line-continuation). Arguments and Properties
may be interspersed in any order, much like is common with positional
arguments vs options in command line tools.

Arguments are ordered relative to each other and that order must be preserved
in order to maintain the semantics.

By contrast, Property order _SHOULD NOT_ matter to implementations.
[Children](#children-block) should be used if an order-sensitive key/value
data structure must be represented in KDL.

Nodes _MAY_ be prefixed with `/-` to "comment out" the entire node, including
its properties, arguments, and children, and make it act as plain whitespace,
even if it spreads across multiple lines.

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
codepoints other than [non-identifier characters](#non-identifier-characters),
so long as this doesn't produce something confusable for a [Number](#number).
Identifiers are terminated by [Whitespace](#whitespace) or
[Newlines](#newline).

### Non-initial characters

The following characters cannot be the first character in a bare
[Identifier](#identifier):

* Any decimal digit (0-9)
* Any [non-identifier characters](#non-identifier-characters)

### Non-identifier characters

The following characters cannot be used anywhere in a bare
[Identifier](#identifier):

* Any codepoint with hexadecimal value `0x20` or below.
* Any codepoint with hexadecimal value higher than `0x10FFFF`.
* Any of "\\<>{};[]()=,\""

### Line Continuation

Line continuations allow [Nodes](#node) to be spread across multiple lines.

A line continuation is one or more [whitespace](#whitespace) characters,
followed by a `\` character. This character can then be followed by more
[whitespace](#whitespace) and must be terminated by a [Newline](#newline)
(including the Newline that is part of single-line comments).

Following a line continuation, processing of a Node can continue as usual.

#### Example
```kdl
my-node 1 2 \  // comments are ok after \
        3 4    // This is the actual end of the Node.
```

### Property

A Property is a key/value pair attached to a [Node](#node). A Property is
composed of an [Identifier](#identifier) or a [String](#string), followed
immediately by a `=`, and then a [Value](#value).

Properties should be interpreted left-to-right, with rightmost properties with
identical names overriding earlier properties. That is:

```
node a=1 a=2
```

In this example, the node's `a` value must be `2`, not `1`.

No other guarantees about order should be expected by implementers.
Deserialized representations may iterate over properties in any order and
still be spec-compliant.

Properties _MAY_ be prefixed with `/-` to "comment out" the entire token and
make it act as plain whitespace, even if it spreads across multiple lines.

### Argument

An Argument is a bare [Value](#value) attached to a [Node](#node), with no
associated key. It shares the same space as [Properties](#properties).

A Node may have any number of Arguments, which should be evaluated left to
right. KDL implementations _MUST_ preserve the order of Arguments relative to
each other (not counting Properties).

Arguments _MAY_ be prefixed with `/-` to "comment out" the entire token and
make it act as plain whitespace, even if it spreads across multiple lines.

### Example

```kdl
my-node 1 2 3 "a" "b" "c"
```

### Value

A value is either: a [String](#string), a [Raw String](#raw-string), a
[Number](#number), a [Boolean](#boolean), or [Null](#null)

Values _MUST_ be either [Arguments](#argument) or values of
[Properties](#property).

### String

Strings in KDL represent textual [Values](#value). They are delimited by `"`
on either side of any number of literal string characters except unescaped
`"` and `\`. This includes literal [Newline](#newline) characters, which means a
String Value can encompass multiple lines without behaving like a Newline for
[Node](#node) parsing purposes.

Strings _MUST_ be represented as UTF-8 values.

In addition to literal code points, a number of "escapes" are supported.
"Escapes" are the character `\` followed by another character, and are
interpreted as described in the following table:

| Name                          | Escape | Code Pt  |
|-------------------------------|--------|----------|
| Line Feed                     | `\n`   | `U+000A` |
| Carriage Return               | `\r`   | `U+000D` |
| Character Tabulation (Tab)    | `\t`   | `U+0009` |
| Reverse Solidus (Backslash)   | `\\`   | `U+005C` |
| Solidus (Forwardslash)        | `\/`   | `U+002F` |
| Quotation Mark (Double Quote) | `\"`   | `U+0022` |
| Backspace                     | `\b`   | `U+0008` |
| Form Feed                     | `\f`   | `U+000C` |
| Unicode Escape                | `\u{(1-6 hex chars)}` | Code point described by hex characters, up to `10FFFF` |

### Raw String

Raw Strings in KDL are much like [Strings](#string), except they do not
support `\`-escapes. They otherwise share the same properties as far as
literal [Newline](#newline) characters go, and the requirement of UTF-8
representation.

Raw String literals are represented as `r`, followed by zero or more `#`
characters, followed by `"`, followed by any number of UTF-8 literals. The string is then
closed by a `"` followed by a _matching_ number of `#` characters. This means
that the string sequence `"` or `"#` and such must not match the closing `"`
with the same or more `#` characters as the opening `r`.

#### Example

```kdl
just-escapes r"\n will be literal"
quotes-and-escapes r#"hello\n\r\asd"world"#
```

### Number

Numbers in KDL represent numerical [Values](#value). There is no logical
distinction in KDL between real numbers, integers, and floating point numbers.
It's up to individual implementations to determine how to represent KDL
numbers.

There are four syntaxes for Numbers: Decimal, Hexadecimal, Octal, and Binary.

* All numbers may optionally start with one of `-` or `+`, which determine whether they'll be positive or negative.
* Binary numbers start with `0b` and only allow `0` and `1` as digits, which may be separated by `_`. They represent numbers in radix 2.
* Octal numbers start with `0o` and only allow digits between `0` and `7`, which may be separated by `_`. They represent numbers in radix 8.
* Hexadecimal numbers start with `0x` and allow digits between `0` and `9`, as well as letters `A` through `F`, in either lower or upper case, which may be separated by `_`. They represent numbers in radix 16.
* Decimal numbers are a bit more special:
    * They have no radix prefix.
    * They use digits `0` through `9`, which may be separated by `_`.
    * They may optionally include a decimal separator `.`, followed by more digits.
    * They may optionally be followed by `E` or `e`, an optional `-` or `+`, and more digits, to represent an exponent value.

### Boolean

A boolean [Value](#value) is either the symbol `true` or `false`. These
_SHOULD_ be represented by implementation as boolean logical values, or some
approximation thereof.

#### Example

```kdl
my-node true value=false
```

### Null

The symbol `null` represents a null [Value](#value). It's up to the
implementation to decide how to represent this, but it generally signals the
"absence" of a value. It is reasonable for an implementation to ignore null
values altogether when deserializing.

#### Example

```kdl
my-node null key=null
```

### Whitespace

The following characters should be treated as non-[Newline](#newline) [white
space](https://www.unicode.org/Public/UCD/latest/ucd/PropList.txt):

| Name                 | Code Pt |
|----------------------|---------|
| Character Tabulation | `U+0009`  |
| Space                | `U+0020`  |
| No-Break Space       | `U+00A0`  |
| Ogham Space Mark     | `U+1680`  |
| En Quad              | `U+2000`  |
| Em Quad              | `U+2001`  |
| En Space             | `U+2002`  |
| Em Space             | `U+2003`  |
| Three-Per-Em Space   | `U+2004`  |
| Four-Per-Em Space    | `U+2005`  |
| Six-Per-Em Space     | `U+2006`  |
| Figure Space         | `U+2007`  |
| Punctuation Space    | `U+2008`  |
| Thin Space           | `U+2009`  |
| Hair Space           | `U+200A`  |
| Narrow No-Break Space| `U+202F`  |
| Medium Mathematical Space | `U+205F`  |
| Ideographic Space    | `U+3000`  |

### Newline

The following characters [should be treated as new
lines](https://www.unicode.org/versions/Unicode13.0.0/ch05.pdf):

| Acronym | Name            | Code Pt |
|---------|-----------------|---------|
| CR      | Carriage Return | `U+000D`  |
| LF      | Line Feed       | `U+000A`  |
| CRLF    | Carriage Return and Line Feed | `U+000D` + `U+000A` |
| NEL     | Next Line       | `U+0085`  |
| FF      | Form Feed       | `U+000C`  |
| LS      | Line Separator  | `U+2028`  |
| PS      | Paragraph Separator | `U+2029` |

Note that for the purpose of new lines, CRLF is considered _a single newline_.

## Full Grammar

```
nodes := linespace* (node nodes?)? linespace*

node := ('/-' ws*)? identifier (node-space node-space* node-props-and-args)* (node-space* node-children ws*)? node-space* node-terminator
node-props-and-args := ('/-' ws*)? (prop | value)
node-children := ('/-' ws*)? '{' nodes '}'
node-space := ws* escline ws* | ws+
node-terminator := single-line-comment | newline | ';' | eof

identifier := string | bare-identifier
bare-identifier := (identifier-char - digit - sign) identifier-char* | sign ((identifier-char - digit) identifier-char*)?
identifier-char := unicode - linespace - [\(){}<>;[]=,"]
prop := identifier '=' value
value := string | number | boolean | 'null'

string := raw-string | escaped-string
escaped-string := '"' character* '"'
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

hex := sign? '0x' hex-digit (hex-digit | '_')*
octal := sign? '0o' [0-7] [0-7_]*
binary := sign? '0b' ('0' | '1') ('0' | '1' | '_')*

boolean := 'true' | 'false'

escline := '\\' ws* (single-line-comment | newline)

linespace := newline | ws | single-line-comment

newline := See Table (All line-break white_space)

ws := bom | unicode-space | multi-line-comment

bom := '\u{FFEF}'

unicode-space := See Table (All White_Space unicode characters which are not `newline`)

single-line-comment := '//' ^newline+ newline
multi-line-comment := '/*' (commented-block | multi-line-comment) '*/'
commented-block := ('*' [^\/] | [^*])*
```
