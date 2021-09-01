# KDL Query Language Spec

This is loosely based on CSS selectors, but without the web-specific stuff.

## Selection operators

* `a > b`: Selects any `b` element that is a direct child of an `a` element.
* `a b`: Selects any `b` element that is a _descendant_ of an `a` element.
* `a b || a c`: Selects all `b` and `c` elements that are descendants of an `a` element. Any selector may be on either side of the `||`. Multiple `||` are supported.
* `a + b`: Selects any `b` element that is placed immediately after a sibling `a` element.
* `a ~ b`: Selects any `b` element that follows an `a` element as a sibling, either immediately or later.
* `[accessor()]`: Selects any element, filtered by [an accessor](#accessors). (`accessor()` is a placeholder, not an actual accessor)
* `a[accessor()]`: Selects any `a` element, filtered by an accessor.
* `[]`: Selects any element (a )

## Matchers

* `top()`: Returns all toplevel children of the current document.
* `top() > []`: Equivalent to `top()` on its own.
* `[val()]`: Selects any element with a value.
* `[val(1)]`: Selects any element with a second value.

Attribute matchers support certain binary operators:

* `[val() = 1]`: Selects any element whose first value is 1.
* `[prop(name) = 1]`: Selects any element with a property `name` whose value is 1.
* `[name = 1]`: Equivalent to the above.
* `[name() = "hi"]`: Selects any element whose _node name_ is "hi". Equivalent to just `hi`, but more useful when using string operators.
* `[val() != 1]`: Selects any element whose first value exists, and is not 1.

The following operators work with any `val()`, `prop()`, or `name()` values.
If the value is not of the same type, the operator will always fail ("1" is
never coerced to 1, and there is no "universal" ordering across all types.):

* `[val() > 1]`: Selects any element whose first value is greater than 1.
* `[val() >= 1]`: Selects any element whose first value is greater than or equal to 1.
* `[val() < 1]`: Selects any element whose first value is less than 1.
* `[val() <= 1]`: Selects any element whose first value is less than or equal to 1.

The following operators work only with string `val()`, `prop()`, or `name()` values. If the value is not a string, the matcher will always fail:

* `[val() ^= "foo"]`: Selects any element whose first value starts with "foo".
* `[val() $= "foo"]`: Selects any element whose first value ends with "foo".
* `[val() *= "foo"]`: Selects any element whose first value contains "foo".

## Map Operator

KQL implementations MAY support a "map operator", `=>`, that allows selection
of specific parts of the selected notes, essentially "mapping" over a
selector's result set.

Only a single map operator may be used, and it must be the last element in a
selector string.

The map operator's right hand side is either an [`accessor`](#accessors) on
its own, or a tuple of accessors, denoted by a comma-separated list wrapped in
`()` (for example, `(a, b, c)`).

## Accessors

Accessors access/extract specific parts of a node. They are used with the [map
operator](#map-operator), and have syntactic overlap with some
[matchers](#matchers).

* `name()`: Returns the name of the node itself.
* `val(2)`: Returns the third value in a node.
* `val()`: Equivalent to `val(0)`.
* `prop(foo)`: Returns the value of the property `foo` in the node.
* `foo`: Equivalent to `prop(foo)`.
* `props()`: Returns all properties of the node as an object.
* `values()`: Returns all values of the node as an array.

## Examples

Given this document:

```kdl
package {
    name "foo"
    version "1.0.0"
    dependencies platform="windows" {
        winapi "1.0.0" path="./crates/my-winapi-fork"
    }
    dependencies {
        miette "2.0.0" dev=true
    }
}
```

Then the following queries are valid:

* `package name` -> fetches the `name` node itself
* `top() > package name` -> fetches the `name` node, guaranteeing that `package` is in the document root.
* `dependencies` -> deep-fetches both `dependencies` nodes
* `dependencies[platform]` -> fetches any dependencies nodes with a `platform` prop (just the one, in this case)
* `dependencies[prop(platform)]` -> Identical to the above. Plain identifiers are equivalent to `prop(<identifier>)`.
* `dependencies > any()` -> fetches all direct-child nodes of any `dependencies`
  nodes in the document. In this case, it will fetch both `miette` and
  `winapi` nodes.

There is an additional `=>` selector (called the "map selector") that MAY be
implemented, which will allow extracting/selecting arbitrary data _from_
nodes, instead of returning the nodes themselves:

* `package name => val(0)` -> `["foo"]`. (The `0` is optional if you just want the first `val()`)
* `dependencies[platform] => platform` -> `["windows"]`
* `dependencies > [] => (name(), val(), path)` -> `[("winapi", "1.0.0", "./crates/my-winapi-fork"), ("miette", "2.0.0", None)]`
* `dependencies > [] => (name(), values(), props())` -> `[("winapi", ["1.0.0"], {"platform": "windows"}), ("miette", ["2.0.0"], {"dev": true})]`
