# KDL Changelog

## 2.0.0 (2022-08-28)

### Grammar

* Solidus/Forward slash (`/`) is no longer an escaped character.
* Single line comments (`//`) can now be immediately followed by a newline.
* All literal whitespace following a `\` in a string is now discarded.
* Vertical tabs (`U+000B`) are now considered to be whitespace.
* Identifiers can't start with `r#`, so they're easy to distinguish from raw
  strings. (They already similarly can't start with a digit, or a sign+digit,
  so they're easy to distinguish from numbers.)
* The grammar syntax itself has been described, and some confusing definitions
  in the grammar have been fixed accordingly (mostly related to escaped
  characters).
* `,`, `<`, and `>` are not legal identifier characters. They were previously
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
* `#` is an illegal initial identifier character, but is allowed in other
  places in identifiers.
* Line continuations can be followed by an EOF now, instead of requiring a
  newline (or comment). `node \<EOF>` is now a legal KDL document.

### KQL

* There's now a _required_ descendant selector (`>>`), instead of using plain
  spaces for that purpose.
* The "any sibling" selector is now `++` instead of `~`, for consistency with
  the new descendant selector.
* Map operators have been removed entirely.
