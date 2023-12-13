JSON-in-KDL (JiK)
=================

This specification describes a canonical way to losslessly encode [JSON](https://json.org) in [KDL](https://kdl.dev). While this isn't a very useful thing to want to do on its own, it's occasionally useful when using a KDL toolchain while speaking with a JSON-consuming or -emitting service.

This is version 4.0.0 of JiK.

JSON-in-KDL (JiK from now on) is a kdl microsyntax consisting of named nodes that represent objects, arrays, or literal values.

----

There are two ways to write a JSON literal into JiK:

* As a node with any nodename and a single argument, like `- #true` (for the JSON `true`) or `foo 5` (for the JSON `5`).
* When nested in arrays or objects, literals can usually be written as arguments (for array nodes) or properties (for object nodes). See below for details.

----

JSON arrays are represented in JiK as a node with any nodename, with zero or more arguments and/or zero or more children with `-` nodenames.

Arguments can encode literals - for example, the JSON `[1, 2, 3]` can be written in JiK as `- 1 2 3`.

Children can encode literals and/or nested arrays and objects. For example, the JSON `[1, [true, false], 3]` can be written in JiK as:

```kdl
- {
	- 1
	- #true #false
	- 3
}
```

The arguments and/or children, taken in order, represent the items of the array.

Arguments and children can be mixed, if desired. The preceding example could also be written as:

```kdl
- 1 {
	- #true #false
	- 3
}
```

Two otherwise-ambiguous cases must be manually annotated with an `(array)` type annotation:

* A single-element array (such as `[1]`) written using arguments (as `- 1`) would be ambiguous with a literal node. 
	To indicate this is an array, it must be written as `(array)- 1`
	(Or rewritten to use child nodes, like `- { - 1 }`.)
* An empty array (JSON `[]`) must use the `(array)` type annotation, like `(array)-`.

The `(array)` type annotation can be used on any other valid array node if desired, but has no effect in such cases.

----

JSON objects are represented in JiK as a node with any nodename, with zero or more properties and/or zero or more children with any nodenames.

Properties can encode literals - for example, the JSON `{"foo": 1, "bar": true}` can be written in JiK as `- foo=1 bar=#true`.

Children can encode literals and/or nested arrays and objects,
using the nodename for the item's key. 

For example, the JSON `{"foo": 1, "bar": [2, {"baz": 3}], "qux":4}` can be written in JiK as:

```kdl
- {
	foo 1
	bar 2 {
		- baz=3
	}
	qux 4
}
```

As with arrays, child nodes and properties can be mixed, so the preceding example could have been written as:

```kdl
- foo=1 {
	bar 2 {
		- baz=3
	}
	qux 4
}
```

Or, so long as the exact order of properties isn't meaningful (it's not *meant* to be in JSON),
*all* the literal-valued keys can be pulled up into properties,
leaving children nodes solely for nested arrays and objects:

```kdl
- foo=1 qux=4 {
	bar 2 {
		- baz=3
	}
}
```

The properties and/or children of the node represent the items of the object,
with the property names and child nodenames as each item's key.
All "keys" in an object node must be unique.

As with arrays, there are two ambiguous cases that must be manually annoted with the `(object)` type annotation:

* An object containing a single item whose key is "-" (like `{"-": 1}`) written using children (like `- { - 1 }`)
	would be ambiguous with an array node.
	To indicate this is an object, it must be written as `(object)- { - 1 }`.
	(Or, if the sole item's value is a literal, as in this example,
	it can be rewritten to use properties, as `- -=1`.)
* An empty object (JSON `{}`) must use the `(object)` type annotation, like `(object)-`.

As with array nodes, `(object)` can be used on any valid object node if desired.

----

Converting JiK back to JSON is a trivial process: literal nodes are encoded as their literal value; array nodes are encoded as their items, comma-separated and surrounded with `[]`; object nodes are encoded as their key/value pairs, comma-separated and surrounded with `{}`.

Only valid JiK nodes can be encoded to JSON; if a JiK document contains an invalid node, the entire document must fail to encode, rather than "guessing" at the intent. As well, a JiK document must contain only a single top-level node to be valid, unless the output is intended to be a JSON stream, in which case arbitrary numbers of nodes are allowed, each a separate JSON value.

* A literal node is valid if it contains a single unnamed argument.

* An array node is valid if it contains only unnamed arguments and/or child nodes named "-". If it contains no arguments and no child nodes, its nodename *must* have the `(array)` type annotation.

* An object node is valid if it contains only named properties and/or child nodes. Additionally, all "keys" must be unique within the node, whether they're encoded as property names or child node names. If it contains no properties and no child nodes, its nodename *must* have the `(object)` type annotation.

----

Note that, outside of array/object items, the nodename is not meaningful in JiK.
For simplicity, this document uses `-` for all such nodenames
(and it is recommended that an automated JSON-to-KDL converter do the same),
but this means it is possible to write a JiK object as meaningful KDL
and embed it within a larger KDL document.

Here's a fictitious example describing an HTTP request with a JSON body, 
where the `body` node is an embedded JiK node
that nevertheless reads as fairly natural KDL.

```kdl
request "/api/cart" method="PUT" {
	body {
		items {
			- id=1234 amount=1
			- id=2341 amount=2 {
				options {
					color "red"
					size "XXL"
				}
			}
		}
	}
}
```

The `body` node represents the JSON object

```json
{
	"items": [
		{"id": 1234, "amount": 1},
		{"id": 2341, "amount": 2, "options": {"color": "red", "size": "XXL"}}
	]
}
```
