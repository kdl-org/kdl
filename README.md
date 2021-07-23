# The KDL Document Language

KDL is a document language with xml-like semantics that looks like you're
invoking a bunch of CLI commands! It's meant to be used both as a
serialization format and a configuration language, much like JSON, YAML, or
XML.

There's a living [specification](SPEC.md), as well as various
[implementations](#implementations). You can also check out the [FAQ](#faq) to
answer all your burning questions!

The language is based on [SDLang](https://sdlang.org), with a number of
modifications and clarifications on its syntax and behavior.

## Design and Discussion

KDL is still extremely new, and discussion about the format should happen over
on the [discussions page](https://github.com/kdl-org/kdl/discussions). Feel
free to jump in and give us your 2 cents!

## Implementations

* Rust: [kdl-rs](https://github.com/kdl-org/kdl-rs)
* JavaScript: [kdljs](https://github.com/kdl-org/kdljs)
* Ruby: [kdl-rb](https://github.com/danini-the-panini/kdl-rb)
* Dart: [kdl-dart](https://github.com/danini-the-panini/kdl-dart)
* Java: [kdl4j](https://github.com/hkolbeck/kdl4j)
* PHP: [kdl-php](https://github.com/kdl-org/kdl-php)
* Python: [kdl-py](https://github.com/daeken/kdl-py)

## Editor Support

* [VS Code](https://marketplace.visualstudio.com/items?itemName=kdl-org.kdl&ssr=false#review-details)

## Overview

### Basics

A KDL node is a node name, followed by zero or more "arguments", and
children.

```kdl
title "Hello, World"
```

You can also have multiple values in a single node!

```kdl
bookmarks 12 15 188 1234
```

Nodes can have properties.

```kdl
author "Alex Monad" email="alex@example.com" active=true
```

And they can have nested child nodes, too!

```kdl
contents {
  section "First section" {
    paragraph "This is the first paragraph"
    paragraph "This is the second paragraph"
  }
}
```

Nodes without children are terminated by a newline, a semicolon, or the end of
a file stream:

```kdl
node1; node2; node3;
```

### Values

KDL supports 4 data types:

* Strings: `"hello world"`
* Numbers: `123.45`
* Booleans: `true` and `false`
* Null: `null`

#### Strings
It supports two different formats for string input: escaped and raw.

```kdl
node "this\nhas\tescapes"
other r"C:\Users\zkat\"
```
Both types of string can be multiline as-is, without a different syntax:

```kdl
string "my
multiline
value"
```

And for raw strings, you can add any number of # after the r and the last " to
disambiguate literal " characters:

```kdl
other-raw r#"hello"world"#
```

#### Numbers

There's 4 ways to represent numbers in KDL. KDL does not prescribe any
representation for these numbers, and it's entirely up to individual
implementations whether to represent all numbers with a single type, or to
have different representations for different forms.

KDL has regular decimal-radix numbers, with optional decimal part, as well as
an optional exponent.

```kdl
num 1.234e-42
```

And using the appropriate prefix, you can also enter hexadecimal, octal, and
binary literals:

```kdl
my-hex 0xdeadbeef
my-octal 0o755
my-binary 0b10101101
```

Finally, all numbers can have underscores to help readability:

```kdl
bignum 1_000_000
```

### Comments

KDL supports C-style comments, both line-based and multiline. Multiline
comments can be nested.

```kdl
// C style

/*
C style multiline
*/

tag /*foo=true*/ bar=false

/*/*
hello
*/*/
```

On top of that, KDL supports `/-` "slashdash" comments, which can be used to
comment out individual nodes, arguments, or children:

```kdl
// This entire node and its children are all commented out.
/-mynode "foo" key=1 {
  a
  b
  c
}

mynode /-"commented" "not commented" /-key="value" /-{
  a
  b
}
```

### More Details

```kdl
// Nodes can be separated into multiple lines
title \
  "Some title"


// Files must be utf8 encoded!
smile "😁"

// Instead of anonymous nodes, nodes and properties can be wrapped
// in "" for arbitrary node names.
"!@#$@$%Q#$%~@!40" "1.2.3" "!!!!!"=true

// The following is a legal bare identifier:
foo123~!@#$%^&*.:'|/?+ "weeee"

// And you can also use unicode!
ノード　お名前="☜(ﾟヮﾟ☜)"

// kdl specifically allows properties and values to be
// interspersed with each other, much like CLI commands.
foo bar=true "baz" quux=false 1 2 3
```

## Design Principles

1. Maintainability
1. Flexibility
1. Cognitive simplicity and Learnability
1. Ease of de/serialization
1. Ease of implementation

## FAQ

#### How do you pronounce KDL?

Same as "cuddle".

#### Why yet another document language?

Because nothing out there felt quite right. The closest one I found was
SDLang, but that had some design choices I disagreed with.

#### Ok, then, why not SDLang?

SDLang is designed for use cases that are not interesting to me, but are very
relevant to the D-lang community. KDL is very similar in many ways, but is
different in the following ways:

* The grammar and expected semantics are [well-defined and specified](SPEC.md).
* There is only one "number" type. KDL does not prescribe representations.
* Slashdash (`/-`) comments are great and useful!
* I am not interested in having first-class date types, and SDLang's are very
  non-standard.
* Values and properties can be interspersed with each other, rather than one
  having to follow the other.
* KDL does not have a first-class binary data type. Just use strings with base64.
* All strings in KDL are multi-line, and raw strings are written with
  Rust-style syntax (`r"foo"`), instead of backticks.
* KDL identifiers can use UTF-8 and are much more lax about symbols than SDLang.
* KDL does not support "anonymous" nodes.
* Instead, KDL supports arbitrary identifiers for node names and attribute
  names, meaning you can use arbitrary strings for those: `"123" "value"=1` is
  a valid node, for example. This makes it easier to use KDL for
  representing arbitrary key/value pairs.

#### Have you seen that one XKCD comic about standards?

Yes. I have. Please stop linking me to it.

#### What about YAML?

YAML is a great, widespread language. Unlike KDL, which is node-based (like
XML or HTML), it's based on map and array data structures, which can provide
an easier serialization experience in some cases.

At the same time, YAML can be ambiguous about what types the data written into
it is. There's also a persistent issue where very large YAML files become
unmanageable, especially due to the significant indentation feature.

KDL is designed to avoid these particular pitfalls by always being explicit
about types, and having clearly-delimited scope (and the ability to
auto-indent/auto-format). Syntax errors are easier to catch, and large files
are (hopefully!) much more manageable.

#### What about JSON?

JSON is a great serialization language, but it can be very difficult to use as
a human configuration language. This is largely due to its very specific, very
strict syntax, as well as its lack of support for comments.

KDL, on the other hand, has great comment support, and has a much more
forgiving syntax without being so flexible as to allow certain classes of
unfortunate mistakes. It also has much more flexibility around how to
represent data.

If you need to interoperate with a service that consumes or emits JSON, or for some other reason have need to write "JSON in KDL", [we have JiK, an official microsyntax for losslessly encoding JSON](JSON-IN-KDL.md).

#### What about TOML?

It nests very poorly. It doesn't fare well with large files.

#### What about XML?

XML is actually pretty fantastic, and has long been a standard for data
exchange across many industries. At the same time, XML is known to be very
verbose, and editing it involves writing (and updating) matching tags. Another
large pitfall with XML is its lack of direct support for arbitrary string
key/value pairs, so what would be a simple `foo: x` in some languages has to
be represented as `<entry name="foo" value="x" />` or something similar. XML
also functions great as a **markup** language. That is, it is easy to
intersperse with text, like HTML.

KDL, just like XML, is a node/element-based language, but with much more
lightweight syntax. It also adds the ability to apply anonymous values
directly to a node, rather than as children. That is, `nodename 1 2 3` instead
of `<element><child>1</child><child>2</child>(etc)</element>`. This can make
it much more manageable and readable as a human configuration language, and is
also less verbose when exchanging documents across APIs!

Finally, KDL is **not** a markup language. XML or HTML do a much better job of
"marking up" a text document with special tags, although KDL can still be
useful for templating engines that want to be more strict about text
fragments.

## License

<a rel="license" href="http://creativecommons.org/licenses/by-sa/4.0/"><img alt="Creative Commons License" style="border-width:0" src="https://i.creativecommons.org/l/by-sa/4.0/88x31.png" /></a><br />This work is licensed under a <a rel="license" href="http://creativecommons.org/licenses/by-sa/4.0/">Creative Commons Attribution-ShareAlike 4.0 International License</a>.

This license applies to the text and assets _in this repository_.
Implementations of this specification are not "derivative works", and thus are
not bound by the restrictions of CC-BY-SA.
