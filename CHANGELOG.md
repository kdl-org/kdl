# KDL Changelog

## 2.0.0-draft.7 (2024-12-10)

* `node-space` is now allowed as whitespace after a `slashdash`, meaning line
  continuations will work now.
* One or two consecutive double-quotes are now allowed in the bodies of
  multi-line quoted strings, without needing to be escaped.
* Grammar has been fixed to disallow raw strings like `#"""#`, which are now
  properly treated as invalid multi-line raw strings (instead of the equivalent of
  `"\""`).
* Test suite has been updated to include a `_fail` suffix in all test cases
  which are expected to fail.
* A slew of additional slashdash and multi-line string compliance tests have
  been added. Have fun. :)
* The organization of string types in the spec prose has been updated to a
  hopefully more helpful structure.


## 2.0.0-draft.6 (2024-12-04)

* Multiline strings, both Raw and Quoted, must now use `"""` instead of a single `"`. Using `"""` for a single-line string is a syntax error.
* Fixed an issue with the `unicode_silly` test case.
* Some rewordings and clarification in the spec prose.
* Slight grammar tweak where the pre-terminator `node-space*` for `node` and `final-node` have been moved into `base-node`.


## 2.0.0-draft.5 (2024-11-28)

* Equals signs other than `=` are no longer supported in properties.
* 128-bit integer type annotations have been added to the list of "well-known"
  type annotations.
* Multiline string escape rules have been tweaked significantly.
* `\s` is now a valid escape within a string, representing a space character.
* Slashdash (`/-`)-compatible locations and related grammar adjusted to be more
  clear and intuitive. This includes some changes relating to whitespace,
  including comments and newlines, which are breaking changes.
* Various updates to test suite to reflect changes.


## 2.0.0 (Unreleased)

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
* Code points under `0x20` (except newline and whitespace code points), code
  points above `0x10FFFF`, Delete control character (`0x7F`), and the [unicode
  "direction control"
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
* Removed a statement in the spec prose that said "It is reasonable for an
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
* Multi-line strings must now use `"""` as delimeters. The opening delimiter must be immediately followed by a newline, and the closing delimiter must be on its own line, prefixed by optional whitespace.
* Multi-line strings are now automatically dedented, according to the common
  whitespace matching the whitespace prefix of the closing line.
* `.1`, `+.1` etc are no longer valid identifiers, to prevent confusion and
  conflicts with numbers.
* Multi-line strings' literal Newline sequences are now normalized to single
  `LF`s.
* `#inf`, `#-inf`, and `#nan` have been added in order to properly support
  IEEE floats for implementations that choose to represent their decimals that
  way.
* Correspondingly, the identifiers `inf`, `-inf`, and `nan` are now syntax
  errors.
* `u128` and `i128` have been added as well-known number type annotations.
* Slashdash (`/-`) -compatible locations adjusted to be more clear and
  intuitive. They can now be used in exactly three different places: before nodes,
  before entire entries, or before entire child blocks.
* Furthermore, The ordering of slashdashed elements has been restricted such
  that a slashdashed child block cannot go before an entry (including slashdashed
  entries).

### KQL

* There's now a _required_ descendant selector (`>>`), instead of using plain
  spaces for that purpose.
* The "any sibling" selector is now `++` instead of `~`, for consistency with
  the new descendant selector.
* Some parsing logic around the grammar has changed.
* Multi- and single-line comments are now supported, as well as line
  continuations with `\`.
* Map operators have been removed entirely.
