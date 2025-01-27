# KDL Query Language Spec

The KDL Query Language is a small language specially tailored for querying KDL
documents to extract nodes and even specific data. It is loosely based on CSS
selectors for familiarity and ease of use. Think of it as CSS Selectors or
XPath, but for KDL!

This document describes KQL `next`. It is unreleased.

## Selectors

Selectors use selection operators to filter nodes that will be returned by an
API using KQL. The main differences between this and CSS selectors are the
lack of `*` (use `[]` instead), the specific syntax for descendants and siblings, and the specific syntax for
[matchers](#matchers) (the stuff between `[` and `]`), which is similar, but not identical to CSS.

* `a > b`: Selects any `b` element that is a direct child of an `a` element.
* `a >> b`: Selects any `b` element that is a _descendant_ of an `a` element.
* `a >> b || a >> c`: Selects all `b` and `c` elements that are descendants of an `a` element. Any selector may be on either side of the `||`. Multiple `||` are supported.
* `a + b`: Selects any `b` element that is placed immediately after a sibling `a` element.
* `a ++ b`: Selects any `b` element that follows an `a` element as a sibling, either immediately or later.
* `[accessor()]`: Selects any element, filtered by [an accessor](#accessors). (`accessor()` is a placeholder, not an actual accessor)
* `a[accessor()]`: Selects any `a` element, filtered by an accessor.
* `[]`: Selects any element.

## Matchers

Matchers are used to filter nodes by their various attributes (such as values,
properties, node names, etc). With the exception of `top()` and `()`, they are all
used inside a `[]` selector. Some matchers are unary, but most of them involve
binary operators.

The `top()` matcher can only be used as the first matcher of a selector. This means
that it cannot be the right operand of the `>`, `>>`, `+`, or `++` operators. As `||`
combines selectors, the `top()` can appear just after it. For instance,
 `a > b || top() > b` is valid, but `a > top()` is not.

* `top()`: Returns all toplevel children of the current document.
* `top() > []`: Equivalent to `top()` on its own.
* `(foo)`: Selects any element whose type annotation is `foo`.
* `()`: Selects any element with any type annotation.
* `[val()]`: Selects any element with a value.
* `[val(1)]`: Selects any element with a second value.
* `[prop(foo)]`: Selects any element with a property named `foo`.
* `[prop]`: Selects any element with a property named `prop`.

Attribute matchers support certain binary operators:

* `[val() = 1]`: Selects any element whose first value is 1.
* `[prop(name) = 1]`: Selects any element with a property `name` whose value is 1.
* `[name = 1]`: Equivalent to the above.
* `[name() = hi]`: Selects any element whose _node name_ is "hi". Equivalent to just `hi`, but more useful when using string operators.
* `[tag() = hi]`: Selects any element whose tag is "hi". Equivalent to just `(hi)`, but more useful when using string operators.
* `[val() != 1]`: Selects any element whose first value exists, and is not 1.

The following operators work with any `val()` or `prop()` values.
If the value is not of the same type, the operator will always fail ("1" is
never coerced to 1, and there is no "universal" ordering across all types.):

* `[val() > 1]`: Selects any element whose first value is greater than 1.
* `[val() >= 1]`: Selects any element whose first value is greater than or equal to 1.
* `[val() < 1]`: Selects any element whose first value is less than 1.
* `[val() <= 1]`: Selects any element whose first value is less than or equal to 1.

The following operators work only with string `val()`, `prop()`, `tag()`, or `name()` values.
If the value is not a string, the matcher will always fail:

* `[val() ^= foo]`: Selects any element whose first value starts with "foo".
* `[val() $= foo]`: Selects any element whose first value ends with "foo".
* `[val() *= foo]`: Selects any element whose first value contains "foo".

The following operators work only with `val()` or `prop()` values. If the value
is not one of those, the matcher will always fail:

* `[val() = (foo)]`: Selects any element whose type annotation is `foo`.

## Examples

Given this document:

```kdl
package {
    name foo
    version "1.0.0"
    dependencies platform=windows {
        winapi "1.0.0" path="./crates/my-winapi-fork"
    }
    dependencies {
        miette "2.0.0" dev=#true integrity=(sri)sha512-deadbeef
    }
}
```

Then the following queries are valid:

* `package >> name`
    * -> fetches the `name` node itself
* `top() > package >> name`
    * -> fetches the `name` node, guaranteeing that `package` is in the document root.
* `dependencies`
    * -> deep-fetches both `dependencies` nodes
* `dependencies[platform]`
    * -> fetches any dependencies nodes with a `platform` prop (just the one, in this case)
* `dependencies[prop(platform)]`
    * -> Identical to the above. Plain identifiers are equivalent to `prop(<identifier>)`.
* `dependencies > []`
    * -> fetches all direct-child nodes of any `dependencies` nodes in the
         document. In this case, it will fetch both `miette` and `winapi` nodes.

## Full Grammar

Rules that are not defined in this grammar are prefixed with `$`, see [the KDL
grammar](https://kdl-org.github.io/kdl/#go.draft-marchan-kdl2.html#full-grammar) for
what they expand to.

```
query-str := $bom? query
query := selector q-ws+ "||" q-ws+ query | selector
selector := filter q-ws+ selector-operator q-ws+ selector-subsequent | filter
selector-subsequent := matchers q-ws+ selector-operator q-ws+ selector-subsequent | matchers
selector-operator := ">>" | ">" | "++" | "+"
filter := "top(" q-ws* ")" | matchers
matchers := type-matcher $string? accessor-matcher* | $string accessor-matcher* | accessor-matcher+
type-matcher := "(" q-ws* ")" | $type
accessor-matcher := "[" q-ws* (comparison | accessor)? q-ws* "]"
comparison := accessor q-ws+ matcher-operator q-ws+ ($type | $string | $number | $keyword)
accessor := "val(" q-ws* $integer q-ws* ")" | "prop(" q-ws* $string q-ws* ")" | "name(" q-ws* ")" | "tag(" q-ws* ")" | "values(" q-ws* ")" | "props(" q-ws* ")" | $string
matcher-operator := "=" | "!=" | ">" | "<" | ">=" | "<=" | "^=" | "$=" | "*="

q-ws := $node-space
```
