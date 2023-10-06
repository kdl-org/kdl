# KDL Changelog

## 2.0.0 (2022-08-28)

### Grammar

* Solidus/Forward slash (`/`) is no longer an escaped character.
* Single line comments (`//`) can now be immediately followed by a newline.
* All literal whitespace following a `\` in a string is now discarded.
* Vertical tabs (`U+000B`) are now considered to be whitespace.

### KQL

* There's now a _required_ descendant selector (`>>`), instead of using plain
  spaces for that purpose.
* The "any sibling" selector is now `++` instead of `~`, for consistency with
  the new descendant selector.
* Map operators have been removed entirely.
