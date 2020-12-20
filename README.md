# KDL - The KDL Document Language

KDL is a document language with xml-like semantics that looks like you're
invoking a bunch of CLI commands!

It's meant to be used both as a serialization format and a configuration
language, and is relatively light on syntax compared to XML.

There's a living [specification](SPEC.md), as well as
[implementations](#implementations). The language is based on
[SDLang](https://sdlang.org), with a number of modifications and
clarifications on its syntax and behavior.

## Design and Discussion

KDL is still extremely new, and discussion about the format should happen over
on the [discussions page](https://github.com/kdl-org/kdl/discussions). Feel
free to jump in and give us your 2 cents!

## Design Principles

1. Maintainability
1. Flexibility
1. Cognitive simplicity and Learnability
1. Ease of de/serialization
1. Ease of implementation

## Implementations

* Rust: [kdl-rs](https://github.com/kdl-org/kdl-rs)

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
ノード　お名前＝"☜(ﾟヮﾟ☜)"

// kdl specifically allows properties and values to be
// interspersed with each other, much like CLI commands.
foo bar=true "baz" quux=false 1 2 3
```

## License

<a rel="license" href="http://creativecommons.org/licenses/by-sa/4.0/"><img alt="Creative Commons License" style="border-width:0" src="https://i.creativecommons.org/l/by-sa/4.0/88x31.png" /></a><br />This work is licensed under a <a rel="license" href="http://creativecommons.org/licenses/by-sa/4.0/">Creative Commons Attribution-ShareAlike 4.0 International License</a>.

