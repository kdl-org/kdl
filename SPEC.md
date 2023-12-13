# KDL Spec

This is the semi-formal specification for KDL, including the intended data
model and the grammar.

This document describes KDL version `1.0.0`. It was released on September 11, 2021.

## Introduction

KDL is a node-oriented document language. Its niche and purpose overlaps with
XML, and as do many of its semantics. You can use KDL both as a configuration
language, and a data exchange or storage format, if you so choose.

The bulk of this document is dedicated to a long-form description of all
[Components](#components) of a KDL document. There is also a much more terse
[Grammar](#full-grammar) at the end of the document that covers most of the
rules, with some semantic exceptions involving the data model.

KDL is designed to be easy to read _and_ easy to implement.

In this document, references to "left" or "right" refer to directions in the
*data stream* towards the beginning or end, respectively; in other words,
the directions if the data stream were only ASCII text. They do not refer
to the writing direction of text, which can flow in either direction,
depending on the characters used.

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

The name may be preceded by a [Type Annotation](#type-annotation) to further
clarify its type, particularly in relation to its parent node. (For example,
clarifying that a particular `date` child node is for the _publication_ date,
rather than the last-modified date, with `(published)date`.)

Following the name are zero or more [Arguments](#argument) or
[Properties](#property), separated by either [whitespace](#whitespace) or [a
slash-escaped line continuation](#line-continuation). Arguments and Properties
may be interspersed in any order, much like is common with positional
arguments vs options in command line tools.

[Children](#children-block) can be placed after the name and the optional
Arguments and Properties, possibly separated by either whitespace or a
slash-escaped line continuation.

Arguments are ordered relative to each other (but not relative to Properties)
and that order must be preserved in order to maintain the semantics.

By contrast, Property order _SHOULD NOT_ matter to implementations.
[Children](#children-block) should be used if an order-sensitive key/value
data structure must be represented in KDL.

Nodes _MAY_ be prefixed with `/-` to "comment out" the entire node, including
its properties, arguments, and children, and make it act as plain whitespace,
even if it spreads across multiple lines.

Finally, a node is terminated by either a [Newline](#newline), a semicolon (`;`)
or the end of the file/stream (an `EOF`).

#### Example

```kdl
foo 1 key="val" 3 {
    bar
    (role)baz 1 2
}
```

### Identifier

A bare Identifier is composed of any [Unicode Scalar
Value](https://unicode.org/glossary/#unicode_scalar_value) other than
[non-initial characters](#non-initial-characters), followed by any number of
Unicode Scalar Values other than [non-identifier
characters](#non-identifier-characters), so long as this doesn't produce
something confusable for a [Number](#number). For example, both a
[Number](#number) and an Identifier can start with `-`, but when an Identifier
starts with `-` the second character cannot be a digit. This is precicely
specified in the [Full Grammar](#full-grammar) below.

When Identifiers are used as the values in [Arguments](#argument) and
[Properties](#property), they are treated as strings, just like they are with
node names and property keys.

Identifiers are terminated by [Whitespace](#whitespace) or
[Newlines](#newline).

In all places where Identifiers are used, [Strings](#string) and [Raw
Strings](#raw-string) can be used in the same place, without an Identifier's
character restrictions.

The literal identifiers `true`, `false`, and `null` are illegal identifiers,
and _MUST_ be treated as a syntax error.

### Non-initial characters

The following characters cannot be the first character in a bare
[Identifier](#identifier):

* Any decimal digit (0-9)
* Any [non-identifier characters](#non-identifier-characters)

Additionally, the `-` character can only be used as an initial character if
the second character is *not* a digit. This allows identifiers to look like
`--this`, and removes the ambiguity of having an identifier look like a
negative number.

### Non-identifier characters

The following characters cannot be used anywhere in a bare
[Identifier](#identifier):

* Any of `(){}[]/\="#;`
* Any [Whitespace](#whitespace) or [Newline](#newline).
* Any [disallowed literal code points](#disallowed-literal-code-points) in KDL
  documents.

### Line Continuation

Line continuations allow [Nodes](#node) to be spread across multiple lines.

A line continuation is a `\` character followed by zero or more whitespace
items (including multiline comments) and an optional single-line comment. It
must be terminated by a [Newline](#newline) (including the Newline that is
part of single-line comments).

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

```kdl
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

#### Example

```kdl
my-node 1 2 3 a b c
```

### Children Block

A children block is a block of [Nodes](#node), surrounded by `{` and `}`. They
are an optional part of nodes, and create a hierarchy of KDL nodes.

Regular node termination rules apply, which means multiple nodes can be
included in a single-line children block, as long as they're all terminated by
`;`.

#### Example

```kdl
parent {
    child1
    child2
}

parent { child1; child2; }
```

### Value

A value is either: an [Identifier](#identifier), a [String](#string), a [Raw
String](#raw-string), a [Number](#number), a [Boolean](#boolean), or
[Null](#null)

Values _MUST_ be either [Arguments](#argument) or values of
[Properties](#property).

Values _MAY_ be prefixed by a single [Type Annotation](#type-annotation).

### Type Annotation

A type annotation is a prefix to any [Node Name](#node) or [Value](#value) that
includes a _suggestion_ of what type the value is _intended_ to be treated as,
or as a _context-specific elaboration_ of the more generic type the node name
indicates.

Type annotations are written as a set of `(` and `)` with a single
[Identifier](#identifier) in it. Any valid identifier or string is considered
a valid type annotation. There must be no whitespace between a type annotation
and its associated Node Name or Value.

KDL does not specify any restrictions on what implementations might do with
these annotations. They are free to ignore them, or use them to make decisions
about how to interpret a value.

Additionally, the following type annotations MAY be recognized by KDL parsers
and, if used, SHOULD interpret these types as follows:

#### Reserved Type Annotations for Numbers Without Decimals:

Signed integers of various sizes (the number is the bit size):

* `i8`
* `i16`
* `i32`
* `i64`

Unsigned integers of various sizes (the number is the bit size):

* `u8`
* `u16`
* `u32`
* `u64`

Platform-dependent integer types, both signed and unsigned:

* `isize`
* `usize`

#### Reserved Type Annotations for Numbers With Decimals:

IEEE 754 floating point numbers, both single (32) and double (64) precision:

* `f32`
* `f64`

IEEE 754-2008 decimal floating point numbers

* `decimal64`
* `decimal128`

#### Reserved Type Annotations for Strings:

* `date-time`: ISO8601 date/time format.
* `time`: "Time" section of ISO8601.
* `date`: "Date" section of ISO8601.
* `duration`: ISO8601 duration format.
* `decimal`: IEEE 754-2008 decimal string format.
* `currency`: ISO 4217 currency code.
* `country-2`: ISO 3166-1 alpha-2 country code.
* `country-3`: ISO 3166-1 alpha-3 country code.
* `country-subdivision`: ISO 3166-2 country subdivision code.
* `email`: RFC5322 email address.
* `idn-email`: RFC6531 internationalized email address.
* `hostname`: RFC1132 internet hostname (only ASCII segments)
* `idn-hostname`: RFC5890 internationalized internet hostname (only `xn--`-prefixed ASCII "punycode" segments, or non-ASCII segments)
* `ipv4`: RFC2673 dotted-quad IPv4 address.
* `ipv6`: RFC2373 IPv6 address.
* `url`: RFC3986 URI.
* `url-reference`: RFC3986 URI Reference.
* `irl`: RFC3987 Internationalized Resource Identifier.
* `irl-reference`: RFC3987 Internationalized Resource Identifier Reference.
* `url-template`: RFC6570 URI Template.
* `uuid`: RFC4122 UUID.
* `regex`: Regular expression. Specific patterns may be implementation-dependent.
* `base64`: A Base64-encoded string, denoting arbitrary binary data.

#### Examples

```kdl
node (u8)123
node prop=(regex).*
(published)date "1970-01-01"
(contributor)person name="Foo McBar"
```

### String

Strings in KDL represent textual [Values](#value). They are delimited by `"`
on either side of any number of literal string characters except unescaped
`"` and `\`. This includes literal [Newline](#newline) characters, which means a
String Value can encompass multiple lines without behaving like a Newline for
[Node](#node) parsing purposes.

Strings _MUST_ be represented as UTF-8 values.

Strings _MUST NOT_ include the code points for [disallowed literal
code points](#disallowed-literal-code-points) directly. If needed, they can be
specified with their corresponding `\u{}` escape.

#### Escapes

In addition to literal code points, a number of "escapes" are supported.
"Escapes" are the character `\` followed by another character, and are
interpreted as described in the following table:

| Name                          | Escape | Code Pt  |
|-------------------------------|--------|----------|
| Line Feed                     | `\n`   | `U+000A` |
| Carriage Return               | `\r`   | `U+000D` |
| Character Tabulation (Tab)    | `\t`   | `U+0009` |
| Reverse Solidus (Backslash)   | `\\`   | `U+005C` |
| Quotation Mark (Double Quote) | `\"`   | `U+0022` |
| Backspace                     | `\b`   | `U+0008` |
| Form Feed                     | `\f`   | `U+000C` |
| Unicode Escape                | `\u{(1-6 hex chars)}` | Code point described by hex characters, as long as it represents a [Unicode Scalar Value](https://unicode.org/glossary/#unicode_scalar_value) |
| Whitespace Escape             | See below | N/A   |

##### Escaped Whitespace

In addition to escaping individual characters, `\` can also escape whitespace.
When a `\` is followed by one or more literal whitespace characters, the `\`
and all of that whitespace are discarded. For example, `"Hello World"` and
`"Hello \    World"` are semantically identical. See [whitespace](#whitespace)
and [newlines](#newlines) for how whitespace is defined.

Note that only literal whitespace is escaped; *escaped* whitespace is retained.
For example, these strings are all semantically identical:

```kdl
"Hello\       \nWorld"

    "Hello\n\
    World"

"Hello\nWorld"

"Hello
World"
```

##### Invalid escapes

Except as described in the escapes table, above, `\` *MUST NOT* precede any
other characters in a string.

### Raw String

Raw Strings in KDL are much like [Strings](#string), except they do not
support `\`-escapes. They otherwise share the same properties as far as
literal [Newline](#newline) characters go, and the requirement of UTF-8
representation.

Raw String literals are represented with one or more `#` characters, followed
by `"`, followed by any number of UTF-8 literals. The string is then closed by
a `"` followed by a _matching_ number of `#` characters. This means that the
string sequence `"` or `"#` and such must not match the closing `"` with the
same or more `#` characters as the opening `#`, in the body of the string.

Like Strings, Raw Strings _MUST NOT_ include any of the [disallowed literal
code-points](#disallowed-literal-code-points) as code points in their body.
Unlike with Strings, these cannot simply be escaped, and are thus
unrepresentable when using Raw Strings.

Like Strings, Raw Strings _MUST NOT_ include any of the [disallowed literal
code-points](#disallowed-literal-code-points) as code points in their body.
Unlike with Strings, these cannot simply be escaped, and are thus
unrepresentable when using Raw Strings.

#### Example

```kdl
just-escapes #"\n will be literal"#
quotes-and-escapes ##"hello\n\r\asd"#world"##
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
    * They may optionally include a decimal separator `.`, followed by more digits, which may again be separated by `_`.
    * They may optionally be followed by `E` or `e`, an optional `-` or `+`, and more digits, to represent an exponent value.

### Boolean

A boolean [Value](#value) is either the symbol `#true` or `#false`. These
_SHOULD_ be represented by implementation as boolean logical values, or some
approximation thereof.

#### Example

```kdl
my-node true value=#false
```

### Null

The symbol `#null` represents a null [Value](#value). It's up to the
implementation to decide how to represent this, but it generally signals the
"absence" of a value.

#### Example

```kdl
my-node #null key=#null
```

### Whitespace

The following characters should be treated as non-[Newline](#newline) [white
space](https://www.unicode.org/Public/UCD/latest/ucd/PropList.txt):

| Name                 | Code Pt |
|----------------------|---------|
| Character Tabulation | `U+0009`  |
| Line Tabulation      | `U+000B`  |
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

#### Multi-line comments

In addition to single-line comments using `//`, comments can also be started
with `/*` and ended with `*/`. These comments can span multiple lines. They
are allowed in all positions where [Whitespace](#whitespace) is allowed and
can be nested.

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

### Disallowed Literal Code Points

The following code points may not appear literally anywhere in the document.
They may be represented in Strings (but not Raw Strings) using `\u{}`.

* Any codepoint with hexadecimal value `0x20` or below (various control characters).
* `0x7F` (the Delete control character).
* Any codepoint that is not a [Unicode Scalar Value](https://unicode.org/glossary/#unicode_scalar_value).
* `0x2066-2069` and `0x202A-202E`, the [unicode "direction control" characters](https://www.w3.org/International/questions/qa-bidi-unicode-controls)

## Full Grammar

This is the full official grammar for KDL and should be considered
authoritative if something seems to disagree with the text above. The [grammar
language syntax](#grammar-language) is defined below.

```
document := bom? nodes

nodes := (line-space* node)* line-space*

plain-line-space := newline | ws | single-line-comment
plain-node-space := ws* escline ws* | ws+

line-space := plain-line-space+ ('/-' plain-node-space* node)?
node-space := plain-node-space+ ('/-' plain-node-space* (node-prop-or-arg | node-children))?

required-node-space := node-space* plain-node-space+
optional-node-space := node-space*

base-node := type? optional-node-space identifier (required-node-space node-prop-or-arg)* (required-node-space node-children)?
node := base-node optional-node-space node-terminator
final-node := base-node optional-node-space node-terminator?
node-prop-or-arg := prop | value
node-children := '{' nodes final-node? '}'
node-terminator := single-line-comment | newline | ';' | eof

identifier := string | bare-identifier
bare-identifier := (unambiguous-ident - boolean - 'null') | numberish-ident
unambiguous-ident := (identifier-char - digit - sign) identifier-char*
numberish-ident := sign ((identifier-char - digit) identifier-char*)?
identifier-char := unicode - line-space - [\\/(){};\[\]="#] - disallowed-literal-code-points

keyword := '#' (boolean | 'null')
prop := identifier optional-node-space '=' optional-node-space value
value := type? optional-node-space (identifier | string | number | keyword)
type := '(' optional-node-space identifier optional-node-space ')'

string := raw-string | escaped-string
escaped-string := '"' string-character* '"'
string-character := '\' escape | [^\\"] - disallowed-literal-code-points
escape := ["\\bfnrt] | 'u{' hex-digit{1, 6} '}' | (unicode-space | newline)+
hex-digit := [0-9a-fA-F]

raw-string := '#' raw-string-quotes '#' | '#' raw-string '#'
raw-string-quotes := '"' (unicode - disallowed-literal-code-points) '"'

number := decimal | hex | octal | binary

decimal := sign? integer ('.' integer)? exponent?
exponent := ('e' | 'E') sign? integer
integer := digit (digit | '_')*
digit := [0-9]
sign := '+' | '-'

hex := sign? '0x' hex-digit (hex-digit | '_')*
octal := sign? '0o' [0-7] [0-7_]*
binary := sign? '0b' ('0' | '1') ('0' | '1' | '_')*

boolean := 'true' | 'false'

escline := '\\' ws* (single-line-comment | newline | eof)

newline := See Table (All line-break white_space)

ws := unicode-space | multi-line-comment

bom := '\u{FEFF}'

disallowed-literal-code-points := See Table (Disallowed Literal Code Points)

unicode-space := See Table (All White_Space unicode characters which are not `newline`)

single-line-comment := '//' ^newline* (newline | eof)
multi-line-comment := '/*' commented-block
commented-block := '*/' | (multi-line-comment | '*' | '/' | [^*/]+) commented-block
```

### Grammar language

The grammar language syntax is a combination of ABNF with some regex spice thrown in.
Specifically:

* Single quotes (`'`) are used to denote literal text. `\` within a literal
  string is used for escaping other single-quotes, for initiating unicode
  characters using hex values (`\u{FEFF}`), and for escaping `\` itself
  (`\\`).
* `*` is used for "zero or more", `+` is used for "one or more", and `?` is
  used for "zero or one".
* `()` can be used to group matches that must be matched together.
* `a | b` means `a or b`, whichever matches first. If multipe items are before
  a `|`, they are a single group. `a b c | d` is equivalent to `(a b c) | d`.
* `[]` are used for regex-style character matches, where any character between
  the brackets will be a single match. `\` is used to escape `\`, `[`, and
  `]`. They also support character ranges (`0-9`), and negation (`^`)
* `-` is used for "except for" or "minus" whatever follows it. For example, `a
  - `'x'` means "any `a`, except something that matches the literal `'x'`".
* The prefix `^` means "something that does not match" whatever follows it.
  For example, `^foo` means "must not match `foo`".
