# kdl - Kat's Document Language

kdl is a document language, mostly based on [SDLang](https://sdlang.org), with
xml-like semantics that looks like you're invoking a bunch of CLI commands!

It's meant to be used both as a serialization format and a configuration
language, and is relatively light on syntax compared to XML.

This repository is the place for discussing the [specification](SPEC.md).

## Design and Discussion

kdl is still extremely new, and discussion about the format should happen over
on the [discussions page](https://github.com/kdoclang/kdl/discussions). Feel free
to jump in and give us your 2 cents!

## Design Principles

1. Maintainability
1. Flexibility
1. Cognitive simplicity and Learnability
1. Ease of de/serialization
1. Ease of implementation

These are the guiding principles behind the design of KDL, in order of
importance. These principles will hopefully be useful in tie-breaking and
otherwise directing specific decisions when it comes down to it. They are
intentionally vague when it comes to specifics, but more concrete definitions
for each one will be settled on as the project matures.

## Implementations

* Rust: [kdl-rs](https://github.com/kdl-org/kdl-rs)

## Overview

The basic syntax is similar to SDLang:

```kdl
// This is a node with a single string value
title "Hello, World"

// Multiple values are supported, too
bookmarks 12 15 188 1234

// Nodes can have properties
author "Alex Monad" email="alex@example.com" active=true

// Nodes can be arbitrarily nested
contents {
  section "First section" {
    paragraph "This is the first paragraph"
    paragraph "This is the second paragraph"
  }
}

// Nodes can be separated into multiple lines
title \
  "Some title"

// Comment formats:

// C++ style

/*
C style multiline
*/

tag /*foo=true*/ bar=false
```

But kdl changes a few details:

```kdl
// Files must be utf8 encoded!
smile "😁"

// Instead of anonymous nodes, nodes and properties can be wrapped
// in "" for arbitrary node names.
"!@#$@$%Q#$%~@!40" "1.2.3" "!!!!!"=true

// The following is a legal bare identifier:
foo123~!@#$%^&*.:'|<>/?+ "weeee"

// kdl specifically allows properties and values to be
// interspersed with each other, much like CLI commands.
foo bar=true "baz" quux=false 1 2 3

// strings can be multiline as-is, without a different syntax.
string "my
multiline
value"

// raw/unescaped strings use the "r" prefix on string literals and
// otherwise behave the same, including multiline support.
raw r"C:\Users\kdl"

// You can add any number of # after the r and the last " to
// disambiguate literal " characters.
other-raw r#"hello"world"#

// There is a single decimal number type, much like JSON's.
num 1.234e-42

// Numbers can have underscores to help readability:
bignum 1_000_000

// There is additional support for literal hexadecimal, octal, and binary input.
my-hex 0xdeadbeef
my-octal 0o755
my-binary 0b1010_1101

// You can comment out individual nodes with /-. In the case below, everything
// up until the closing `}` becomes commented.
/-mynode "foo" key=1 {
  a
  b
  c
}

// You can apply /- ("slashdash") comments to individual values, properties,
// or child blocks, too:
mynode /-"commented" "not commented" /-key="value" /-{
  a
  b
}

```

The following SDLang features are removed altogether:

* "Anonymous" nodes
* Binary data literals
* Date/time formats
* `on` and `off` booleans
* Backtick strings
* Semicolons
* Namespaces with `:`
* Shell style (`#`) and Lua-style (`--`) comments
* Distinction between 32/64/128-bit numbers. There's just numbers.

## License

<a rel="license" href="http://creativecommons.org/licenses/by-sa/4.0/"><img alt="Creative Commons License" style="border-width:0" src="https://i.creativecommons.org/l/by-sa/4.0/88x31.png" /></a><br />This work is licensed under a <a rel="license" href="http://creativecommons.org/licenses/by-sa/4.0/">Creative Commons Attribution-ShareAlike 4.0 International License</a>.

