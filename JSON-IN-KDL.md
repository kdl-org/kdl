JSON-in-KDL (JiK)
=================

This specification describes a canonical way to losslessly encode [JSON](https://json.org) in [KDL](https://kdl.dev). While this isn't a very useful thing to want to do on its own, it's occasionally useful when using a KDL toolchain while speaking with a JSON-consuming or -emitting service.

This is version 3.0.0 of JiK.

JSON-in-KDL (JiK from now on) is a kdl microsyntax consisting of named nodes that represent objects, arrays, or literal values.

The name "-" is used for nodes that are nameless, i.e. the top-level node and items in an array.

----

Literal nodes are used to represent a JSON literal, which luckily KDL's literal syntax is a superset of. They contain a single value, the literal they're representing. For example, to represent the JSON literal `true`, you'd write `- true` in JiK.

(In many cases this isn't necessary, and KDL literals can be directly used instead. Literal nodes are necessary only for a top-level literal, or as item in an array.)

----

Array nodes are used to represent a JSON array. They can contain zero or more unnamed arguments, followed by zero or more child nodes; these are taken as the items of the array, in order of appearance.

This means that simple arrays of literals can be written compactly and simply; a JSON array like `[1,2,3]` can be written in JiK as `- 1 2 3`. When an array contains nested arrays or objects, the child nodes are used; a JSON array like `[1, [true, false], 3]` can be written in JiK as:

```kdl
- {
	- 1
	- true false
	- 3
}
```

The two methods of writing children can be mixed, pulling the prefix of the array that is just literals into the arguments of the node. The preceding example could thus also be written as:

```kdl
- 1 {
	- true false
	- 3
}
```

----

Object nodes are used to represent a JSON object. They can contain zero or more named properties, followed by zero or more child nodes; these are taken as the key/value pairs of the object, in order of appearance.

If the value of a key/value pair is a literal, it can be encoded as a named property on the object. For example, the JSON object `{"foo": 1, "bar": true}` could be written in JiK as `- foo=1 bar=true`.

Alternately, key/value pairs can be encoded as child nodes, using a type annotation on the node name to encode the key, and the node itself as the value. The preceding example could instead have been written as:

```kdl
- {
	foo 1
	bar true
}
```

Of course, using children for literals is overly-verbose. It's only necessary when nesting arrays or objects into objects; for example, the JSON object `{"foo": [1, 2, {"bar": 3}], "baz":4}` can be written in JiK as:

```kdl
- {
	foo 1 2 {
		- bar=3
	}
	baz 4
}
```

As with arrays, child nodes and properties can be mixed. The precise order of a JSON object's keys isn't *meant* to be meaningful, so as long as that's true, *all* the keys with literal values can be pulled into the argument list. The preceding example could thus also be written as:

```kdl
- baz=4 {
	foo 1 2 {
		- bar=3
	}
}
```

----

There are two cases where there can be ambiguity between the three kinds of nodes. These can be solved by explicitly marking the node as an array or object using a tag.

An array with a single item cannot be represented using a node with a single value as that would make it a literal node. The `(array)` tag can be used to mark this node as an array instead.  
For example, the node `- true` is the literal `true`, while `(array)- true` is the array `[true]`.

An object with a single property named "-" that is encoded as a child node will be interpreted as an array with a single item. The `(object)` tag can be used to mark this node as an object instead.  
For example, `- { - true; }` is the array `[true]`, while `(object)- { - true; }` is the object `{"-": true}`.

----

Converting JiK back to JSON is a trivial process: literal nodes are encoded as their literal value; array nodes are encoded as their items, comma-separated and surrounded with `[]`; object nodes are encoded as their key/value pairs, comma-separated and surrounded with `{}`.

Only valid JiK nodes can be encoded to JSON; if a JiK document contains an invalid node, the entire document must fail to encode, rather than "guessing" at the intent. As well, a JiK document must contain only a single top-level node to be valid, unless the output is intended to be a JSON stream, in which case arbitrary numbers of nodes are allowed, each a separate JSON value.

* A literal node is valid if it contains a single unnamed argument.

* An array node is valid if it contains only unnamed arguments and/or child nodes named "-".

* An object node is valid if it contains only named properties and/or child nodes. Additionally, all "keys" must be unique within the node, whether they're encoded as property names or type annotations on node names.

----

The name of the top-level JiK node is not taken into account. This allows for using a declarative node name instead of "-".

It is possible to embed JiK inside KDL documents. Here's a fictitious example describing an HTTP request with a JSON body, where the `body` node is an embedded JiK node.

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
