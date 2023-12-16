# KDL Changelog

## 2.0.0 (2022-08-28)

### Grammar

* Solidus/Forward slash (`/`) is no longer an escaped character.
* Space (`U+0020`) can now be written into quoted strings with the `\s`
  escape.
* Single line comments (`//`) can now be immediately followed by a newline.
* All literal whitespace following a `\` in a string is now discarded.
* Vertical tabs (`U+000B`) are now considered to be whitespace.
* The grammar syntax itself has been described, and some confusing definitions
  in the grammar have been fixed accordingly (mostly related to escaped
  characters).
* `,`, `<`, and `>` are now legal identifier characters. They were previously
  reserved for KQL but this is no longer necessary.
* Code points under `0x20`, code points above `0x10FFFF`, Delete control
  character (`0x7F`), and the [unicode "direction control"
  characters](https://www.w3.org/International/questions/qa-bidi-unicode-controls)
  are now completely banned from appearing literally in KDL documents. They
  can now only be represented in regular strings, and there's no facilities to
  represent them in raw strings. This should be considered a security
  improvement.
* Raw strings no longer require an `r` prefix: they are now specified by using
  `#""#`.
* Line continuations can be followed by an EOF now, instead of requiring a
  newline (or comment). `node \<EOF>` is now a legal KDL document.
* `#` is no longer a legal identifier character.
* `null`, `true`, and `false` are now `#null`, `#true`, and `#false`. Using
  the unprefixed versions of these values is a syntax error.
* The spec prose has more explicitly stated that whitespace and newlines are
  not valid identifier characters, even though the grammar already expressed
  this.
* Bare identifiers can now be used as values in Arguments and Properties, and are interpreted as string values.
* The spec prose now more explicitly states that strings and raw strings can
  be used as type annotations.
* A statement in the spec prose that said "It is reasonable for an
  implementation to ignore null values altogether when deserializing". This is
  no longer encouraged or desired.
* Code points have been constrained to [Unicode Scalar
  Values](https://unicode.org/glossary/#unicode_scalar_value) only, including
  values used in string escapes (`\u{}`). All KDL documents and string values
  should be valid UTF-8 now, as was intended.
* The last node in a child block no longer needs to be terminated with `;`,
  even if the closing `}` is on the same line, so this is now a legal node:
  `node {foo;bar;baz}`
* More places allow whitespace (node-spaces, specifically) now. With great
  power comes great responsibility:
  * Inside `(foo)` annotations (so, `( foo )` would be legal (`( f oo )` would
    not be, since it has two identifiers))
  * Between annotations and the thing they're annotating (`(blah) node (thing)
    1 y= (who) 2`)
  * Around `=` for props (`x = 1`)
* The BOM is now only allowed as the first character in a document. It was
  previously treated as generic whitespace.
* Multi-line strings are now automatically dedented, according to the
  least-indented line in the body. Multiline strings and raw strings now must
  have a newline immediately following their opening `"`, and a final newline
  preceding the closing `"`.

### KQL

* There's now a _required_ descendant selector (`>>`), instead of using plain
  spaces for that purpose.
* The "any sibling" selector is now `++` instead of `~`, for consistency with
  the new descendant selector.
* Map operators have been removed entirely.
