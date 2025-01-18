# Contributing

## Mechanics

Contributions can be made by creating pull requests.
The GitHub interface supports creating pull requests using the Edit (✏) button.


## Building the Specification

The specification is written in
[kramdown-rfc](https://github.com/cabo/kramdown-rfc/wiki/Syntax2), which
compiles via [RFCXML](https://authors.ietf.org/rfcxml-vocabulary) to text and
HTML.

You can build the formatted versions or the intermediate RFCXML file using
https://author-tools.ietf.org/ or locally by running `make`. To preserve the
intermediate RFCXML form in a local build, run `make draft-marchan-kdl2.xml`
once.

Command line usage requires that you have the necessary software installed.  See
[the instructions](https://github.com/martinthomson/i-d-template/blob/main/doc/SETUP.md).
