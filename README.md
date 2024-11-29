# The KDL Document Language

> [!WARNING]  
> The main branch of this repository shows the latest v2.0.0 draft, which is a
> work in progress and not considered the "mainline" KDL yet. Most KDL
> implementations in the wild are based on the [v1.0.0
> spec](https://github.com/kdl-org/kdl/tree/1.0.0) instead, so you may want to
> refer to that if you're using KDL today.

KDL is a small, pleasant document language with XML-like node semantics that
looks like you're invoking a bunch of CLI commands! It's meant to be used both
as a serialization format and a configuration language, much like JSON, YAML,
or XML. It looks like this:

```kdl
package {
  name my-pkg
  version "1.2.3"

  dependencies {
    // Nodes can have standalone values as well as
    // key/value pairs.
    lodash "^3.2.1" optional=#true alias=underscore
  }

  scripts {
    // "Raw" and dedented multi-line strings are supported.
    build #"
      echo "foo"
      node -c "console.log('hello, world!');"
      echo "foo" > some-file.txt
      "#
  }

  // `\` breaks up a single node across multiple lines.
  the-matrix 1 2 3 \
             4 5 6 \
             7 8 9

  // "Slashdash" comments operate at the node level,
  // with just `/-`.
  /-this-is-commented {
    this entire node {
      is gone
    }
  }
}
```

There's a living [specification](SPEC.md), as well as various
[implementations](#implementations). You can also check out the [FAQ](#faq) to
answer all your burning questions!

The current version of the KDL spec is `2.0.0-draft.5`.

In addition to a spec for KDL itself, there are also standard specs for [a KDL
Query Language](QUERY-SPEC.md) based on CSS selectors, and [a KDL Schema
Language](SCHEMA-SPEC.md) loosely based on JSON Schema.

The language is based on [SDLang](https://sdlang.org), with a [number of
modifications and clarifications on its syntax and behavior](#why-not-sdlang).

[Play with it in your browser!](https://kdl-play.danini.dev/)

## Design and Discussion

KDL 2.0 design is still in progress. Discussions and questions about the format
should happen over on the [discussions
page](https://github.com/kdl-org/kdl/discussions). Feel free to jump in and give
us your 2 cents!

## Implementations

* Rust: [kdl-rs](https://github.com/kdl-org/kdl-rs), [knuffel](https://crates.io/crates/knuffel/) (latter includes derive macro), and [kaydle](https://github.com/Lucretiel/kaydle) (serde-based)
* JavaScript: [kdljs](https://github.com/kdl-org/kdljs), [@virtualstate/kdl](https://github.com/virtualstate/kdl) (query only, JSX based)
* Ruby: [kdl-rb](https://github.com/danini-the-panini/kdl-rb)
* Dart: [kdl-dart](https://github.com/danini-the-panini/kdl-dart)
* Java: [kdl4j](https://github.com/hkolbeck/kdl4j)
* PHP: [kdl-php](https://github.com/kdl-org/kdl-php)
* Python: [kdl-py](https://github.com/tabatkins/kdlpy), [cuddle](https://github.com/djmattyg007/python-cuddle), [ckdl](https://github.com/tjol/ckdl)
* Elixir: [kuddle](https://github.com/IceDragon200/kuddle)
* XSLT: [xml2kdl](https://github.com/Devasta/XML2KDL)
* Haskell: [Hustle](https://github.com/fuzzypixelz/Hustle)
* .NET: [Kadlet](https://github.com/oledfish/Kadlet)
* C: [ckdl](https://github.com/tjol/ckdl)
* C++: [kdlpp](https://github.com/tjol/ckdl) (part of ckdl, requires C++20)
* OCaml: [ocaml-kdl](https://github.com/Bannerets/ocaml-kdl)
* Nim: [kdl-nim](https://github.com/Patitotective/kdl-nim)
* Common Lisp: [kdlcl](https://github.com/chee/kdlcl)
* Go: [gokdl](https://github.com/lunjon/gokdl), [kdl-go](https://github.com/sblinch/kdl-go)
* Swift: [kdl-swift](https://github.com/danini-the-panini/kdl-swift)
* Crystal: [kdl-cr](https://github.com/danini-the-panini/kdl-cr)
* Lua: [kdlua](https://github.com/danini-the-panini/kdlua)

## Compatibility Test Suite

There is a [compatibility test suite](tests/README.md) available for KDL
implementors to check that their implementations are actually spec-compatible.

The implementations above are not guaranteed to pass this test suite in its
entirety, but in the future, may be required to in order to be included here.

## Editor Support

* [VS Code](https://marketplace.visualstudio.com/items?itemName=kdl-org.kdl&ssr=false#review-details)
* [Sublime Text](https://packagecontrol.io/packages/KDL)
* [vim](https://github.com/imsnif/kdl.vim)
* [neovim](https://github.com/nvim-treesitter/nvim-treesitter)
* [Intellij IDEA](https://plugins.jetbrains.com/plugin/20136-kdl-document-language)

## Overview

### Basics

A KDL node is a node name string, followed by zero or more "arguments", and
children.

```kdl
title "Hello, World"
```

You can also have multiple values in a single node!

```kdl
bookmarks 12 15 188 1234
```

Nodes can have properties, with string keys.

```kdl
author "Alex Monad" email=alex@example.com active=#true
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

* Strings: `unquoted`, `"hello world"`, or `#"hello world"#`
* Numbers: `123.45`
* Booleans: `#true` and `#false`
* Null: `#null`

#### Strings

It supports three different formats for string input: identifiers, quoted, and raw.

```kdl
node1 this-is-a-string
node2 "this\nhas\tescapes"
node3 #"C:\Users\zkat\raw\string"#
```

You don't have to quote strings unless any the following apply:
  * The string contains whitespace.
  * The string contains any of `[]{}()\/#";=`.
  * The string is one of `true`, `false`, `null`, `inf`, `-inf`, or `nan`.
  * The strings starts with a digit, or `+`/`-`/`.`/`-.`,`+.` and a digit.
    (aka "looks like a number")

In essence, if it can get confused for other KDL or KQL syntax, it needs
quotes.

Both types of quoted string can be multiline as-is, without a different
syntax. Additionally, common indentation shared with the line containing the
closing quote will be stripped/dedented:

```kdl
string "
  my
    multiline
  value
  "
```

Raw strings, which do not support `\` escapes and can be used when you want
certain kinds of strings to look nicer without having to escape a lot:

```kdl
exec #"
  echo "foo"
  echo "bar"
  cd C:\path\to\dir
  "#

regex #"\d{3} "[^/"]+""#
```

You can add any number of `#`s before and after the opening and
closing `#` to disambiguate literal closing `#"` sequences:

```kdl
other-raw ##"hello#"world"##
```

#### Numbers

There are 4 ways to represent numbers in KDL. KDL does not prescribe any
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

tag /*foo=#true*/ bar=#false

/*/*
hello
*/*/
```

On top of that, KDL supports `/-` "slashdash" comments, which can be used to
comment out individual nodes, arguments, or child blocks:

```kdl
// This entire node and its children are all commented out.
/-mynode foo key=1 {
  a
  b
  c
}

mynode /-commented "not commented" /-key=value /-{
  a
  b
}
// The above is equivalent to:
mynode "not commented"
```

### Type Annotations

KDL supports type annotations on both values and nodes. These can be
arbitrary, but can be used by individual implementations or use-cases to
constrain KDL's basic types. A number of type names are also reserved to have
specific meanings.

```kdl
numbers (u8)10 (i32)20 myfloat=(f32)1.5 {
  strings (uuid)"123e4567-e89b-12d3-a456-426614174000" (date)"2021-02-03" filter=(regex)#"$\d+"#
  (author)person name=Alex
}
```

### More Details

```kdl
// Nodes can be separated into multiple lines
title \
  "Some title"


// Files must be utf8 encoded!
smile 😁

// Node names and property keys are just strings, so you can write them like
// quoted or raw strings, too!
"illegal{}[]/\\=#;identifier" #"1.2.3"# "#false"=#true

// Identifiers are very flexible. The following is a legal bare identifier:
<@foo123~!$%^&*.:'|?+>

// And you can also use unicode!
ノード　お名前=ฅ^•ﻌ•^ฅ

// kdl specifically allows properties and values to be
// interspersed with each other, much like CLI commands.
foo bar=#true baz quux=#false 1 2 3
```

## Design Principles

1. Maintainability
1. Flexibility
1. Cognitive simplicity and Learnability
1. Ease of de/serialization
1. Ease of implementation

## Compatibility with JSON and XML

There are two specifications for writing KDL that can be losslessly translated
between it and JSON or XML. These specifications define a stricter _subset_ of
KDL that, even if not entirely idiomatic, is still valid and fits into the
data models of the other two languages:

* [JSON in KDL](JSON-IN-KDL.md)
* [XML in KDL](XML-IN-KDL.md)

## FAQ

#### How do you pronounce KDL?

Same as "cuddle".

#### Why yet another document language?

Because nothing out there felt quite right. The closest one I found was
SDLang, but that had some design choices I disagreed with.

<a name="why-not-sdlang"></a>
#### Ok, then, why not SDLang?

SDLang is an excellent base, but I wanted some details ironed out, and some
things removed that only really made sense for SDLang's current use-cases, including
some restrictions about data representation. KDL is very similar in many ways, except:

* The grammar and expected semantics are [well-defined and specified](SPEC.md).
* There is only one "number" type. KDL does not prescribe representations, but
  does have keywords for NaN, infinity, and negative infinity if decimal numbers
  are intended to be represtented as IEEE754 floats.
* Slashdash (`/-`) comments are great and useful!
* Quoteless "identifier" strings are supported. (e.g. `node foo=bar`, vs `node foo="bar"`)
* KDL does not have first-class date or binary data types. Instead, it
  supports arbitrary type annotations for any custom data type you might need:
  `(date)"2021-02-03"`, `(binary)"deadbeefbadc0ffee"`.
* Values and properties can be interspersed with each other, rather than one
  having to follow the other.
* All strings in KDL are multi-line, and multi-line strings are automatically dedented to match their closing quote's indentation level.
* Raw strings are written with `#` (`#"foo\bar"#`), instead of backticks.
* KDL identifiers can use UTF-8 and are more lax about symbols than SDLang.
* KDL does not support "anonymous" nodes.
* Namespaces are not supported, but `:` is a legal identifier character, and applications
  can choose to implement namespaces as they see fit.
* KDL supports arbitrary identifiers for node names and attribute
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

If you need to interoperate with a service that consumes or emits JSON, or for
some other reason have need to write "JSON in KDL", [we have JiK, an official
microsyntax for losslessly encoding JSON](JSON-IN-KDL.md).

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

If you need to interoperate with a service that consumes or emits XML, or for
some other reason have need to write "XML in KDL", [we have XiK, an official
microsyntax for losslessly encoding XML](XML-IN-KDL.md).

## License

<a rel="license" href="http://creativecommons.org/licenses/by-sa/4.0/"><img alt="Creative Commons License" style="border-width:0" src="https://i.creativecommons.org/l/by-sa/4.0/88x31.png" /></a><br />This work is licensed under a <a rel="license" href="http://creativecommons.org/licenses/by-sa/4.0/">Creative Commons Attribution-ShareAlike 4.0 International License</a>.

This license applies to the text and assets _in this repository_.
Implementations of this specification are not "derivative works", and thus are
not bound by the restrictions of CC-BY-SA.

The KDL logo design and files were generously contributed by Timothy Merritt
([@timmybytes](https://github.com/timmybytes)), and are also available under
the same license.
