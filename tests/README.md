# Full Document Test Cases

The `input` folder contains test cases for KDL parsers. The `expected_kdl`
folder contains files with the same name as those in `input` with the expected
output after being run through the parser and printed out again.

If a testcase is intended to fail parsing,
the `input` file _MUST_ have a `_fail` suffix,
and there must be no corresponding file in `expected_kdl`.

## Translation Rules

By necessity, the files in `expected_kdl` are not identical to their
corresponding inputs. They are instead pretty-printed according to the
following rules:

* All comments removed
* Extra empty lines removed except for a newline after the last node
* All nodes should be reformatted without escaped newlines
* Node fields should be `identifier <values> <properties> <children only if non-empty>`
* All values and all children must be in the same order as they were defined.
* Properties must be in _alphabetical order_ and separated by a single space.
* All strings must be represented as regular strings, with appropriate escapes
  for invalid bare characters. That means that raw strings must be converted
  to plain strings, and escaped.
* Any literal newlines or other ascii escape characters in escaped strings
  replaced with their escape sequences.
* All identifiers must be unquoted unless they _must_ be quoted. That means
  `"foo"` becomes `foo`, and `"foo bar"` stays that way.
* Any duplicate properties must be removed, with only the rightmost one
  remaining. This also means duplicate properties must be allowed.
* 4 space indents
* All numbers must be converted to their simplest decimal representation. That
  means that hex, octal, and binary must all be converted to decimals. All
  floats must be represented using `E` notation, with a single digit left of
  the decimal point if the float is less than 1. While parsers are required to
  _consume_ different number syntaxes, they are under no obligation to
  represent numbers in any particular way.

Data may be manipulated as you wish in order to output the expected KDL. This
test suite verifies the ability to **parse**, not specific quirks about
internal representations.

## What to do if a test fails for you

This test suite was originally designed for a pre-1.0 version of the KDL
specification. If you encounter a failure, it's likely that the test suite
will need to be updated, rather than your parser itself. This test suite is
NOT AUTHORITATIVE. If this test suite disagrees with the KDL spec in any way,
the most desirable resolution is to send a PR to this repository to fix the
test itself. Likewise, if you think a test succeeded but should not have,
please send a PR.

If you think the disagreement is due to a genuine error or oversight in the
KDL specification, please open an issue explaining the matter and the change
will be considered for the next version of the KDL spec.
