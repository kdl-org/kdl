# kdl - Kat's Document Language

kdl is a document language, mostly based on [SDLang](https://sdlang.org), with
xml-like semantics that looks like you're invoking a bunch of CLI commands!

It's meant to be used both as a serialization format and a configuration
language, and is relatively light on syntax compared to XML.

## Intro

Most of the syntax is basically the same as SDLang:

```kdl
// This is a node with a single string value
title "Hello, World"

// Multiple values are supported, too
bookmarks 12 15 188 1234

// Nodes can have attributes
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
// Instead of anonymous nodes, nodes and properties can be wrapped
// in "" for arbitrary node names.
"!@#$@$%Q#$%~@!40" "1.2.3" "!!!!!"=true

// The following is a legal bare identifier:
foo123~!@#$%^&*.:'|<> "weeee"

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

// There is additional support for literal hexadecimal, octal, and binary input.
my-hex 0xdeadbeef
my-octal 0o755
my-binary 0b10101101
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
* Distinction between 32/64/128-bit numbers. There's just integers and floats.

## Design and Discussion

kdl is still extremely new, and discussion about the format should happen over
on the [discussions page](https://github.com/zkat/kdl/discussions). Feel free
to jump in and give us your 2 cents!
