# Full Document Test Cases

The `input` folder contains test cases for KDL parsers. The `expected_kdl`
folder contains files with the same name as those in `input` with the expected
output after being run through the parser and printed out again. If there's no
file in `expected_kdl` with a name corresponding to one in `input` it
indicates that parsing for that case should fail.

By necessity, the files in `expected_kdl` are not identical to their
corresponding inputs. They are instead pretty-printed according to the
following rules:

* All comments removed
* Extra empty lines removed except for a newline after the last node
* All nodes should be reformatted without escaped newlines
* Node fields should be `identifier <args in same order> <properties in alpha order by key> <children if non-empty>`
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
