# KDL Changelog

## 2.0.0 (2022-08-28)

### Grammar

* Solidus/Forward slash (`/`) is no longer an escaped character.
* Single line comments (`//`) can now be immediately followed by a newline.
* Blocks of whitespace in strings can now be escaped and discarded.

### KQL

* There's now a _required_ descendant selector (`>>`), instead of using plain
  spaces for that purpose.
* The "any sibling" selector is now `++` instead of `~`, for consistency with
  the new descendant selector.
* Map operators have been removed entirely.
