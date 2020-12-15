# KDL Spec

This is the kinda-formal specification for KDL, including the intended data
model and the grammar.

## Full Grammar

```
nodes := linespace* (node (newline nodes)? linespace*)?

node := identifier (node-space node-argument)* (node-space node-document)? single-line-comment?
node-argument := prop | value
node-children := '{' nodes '}'
node-space := ws* escline ws* | ws+

identifier := [a-zA-Z] [a-zA-Z0-9!$%&'*+\-./:<>?@\^_|~]* | string
prop := identifier '=' value
value := string | raw_string | number | boolean | 'null'

string := '"' character* '"'
character := '\' escape | [^\"]
escape := ["\\/bfnrt] | 'u{' hex-digit{1, 6} '}'
hex-digit := [0-9a-fA-F]

raw-string := 'r' raw-string-hash
raw-string-hash := '#' raw-string-hash '#' | raw-string-quotes
raw-string-quotes := '"' .* '"'

number := decimal | hex | octal | binary

decimal := integer ('.' [0-9]+)? exponent?
exponent := ('e' | 'E') integer
integer := sign? [0-9] [0-9_]*
sign := '+' | '-'

hex := '0x' hex-digit (hex-digit | '_')*
octal := '0o' [0-7] [0-7_]*
binary := '0b' ('0' | '1') ('0' | '1' | '_')*

boolean := 'true' | 'false'

escline := '\\' ws* (single-line-comment | newline)

linespace := newline | ws | single-line-comment

newline := ('\r' '\n') | '\n'

ws := bom | ' ' | '\t' | multi-line-comment

single-line-comment := '//' ('\r' [^\n] | [^\r\n])* newline
multi-line-comment := '/*' ('*' [^\/] | [^*])* '*/'
```
