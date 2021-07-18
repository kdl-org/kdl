JSON-in-KDL (JiK)
=================

This specification describes a canonical way to losslessly encode [JSON](https://json.org) in [KDL](https://kdl.dev). While this isn't a very useful thing to want to do on its own, it's occasionally useful when using a KDL toolchain while speaking with a JSON-consuming or -emitting service.

This is version 1.0.0 of JiK.

JSON-in-KDL (JiK from now on) is a kdl microsyntax consisting of three types of nodes:

* literal nodes, with `_` as the nodename
* array nodes, with `array` as the nodename
* object nodes, with `object` as the nodename

----

Literal nodes are used to represent a JSON literal, which luckily KDL's literal syntax is a superset of. They contain a single value, the literal they're representing. For example, to represent the JSON literal `true`, you'd write `_ true` in JiK.

(In many cases this isn't necessary, and KDL literals can be directly used instead. Literal nodes are necessary only for a top-level literal, or to intersperse literals with arrays or objects inside an array or object node.)

----

Array nodes are used to represent a JSON array. They can contain zero or more unnamed arguments, followed by zero or more child nodes; these are taken as the items of the array, in order of appearance.

This means that simple arrays of literals can be written compactly and simply; a JSON array like `[1,2,3]` can be written in JiK as `array 1 2 3`. When an array contains nested arrays or objects, the child nodes are used; a JSON array like `[1, [true, false], 3]` can be written in JiK as:

```kdl
array {
	_ 1
	array true false
	_ 3
}
```

The two methods of writing children can be mixed, pulling the prefix of the array that is just literals into the arguments of the node. The preceding example could thus also be written as:

```kdl
array 1 {
	array true false
	_ 3
}
```

----

Object nodes are used to represent a JSON object. They can contain zero or more named properties, followed by zero or more child nodes; these are taken as the key/value pairs of the object, in order of appearance.

The named properties of an object node are key/value pairs, used when the value is a literal.

For example, the JSON object `{"foo": 1, "bar": true}` could be written in JiK as `object foo=1 bar=true`.

The children of an object node have a slightly modified syntax: they must contain a string as their first value, giving their "key"; the child itself, besides the "key" argument, is the "value". The preceding example could instead have been written as:

```kdl
object {
	_ "foo" 1
	_ "bar" true
}
```

Of course, using children for literals is overly-verbose. It's only necessary when nesting arrays or objects into objects; for example, the JSON object `{"foo": [1, 2, {"bar": 3}], "baz":4}` can be written in JiK as:

```kdl
object {
	array "foo" 1 2 {
		object bar=3
	}
	_ "baz" 4
}
```

As with arrays, child lists and arguments can be mixed. The precise order of a JSON object's keys isn't *meant* to be meaningful, so as long as that's true, *all* the keys with literal values can be pulled into the argument list. The preceding example could thus also be written as:

```kdl
object baz=4 {
	array "foo" 1 2 {
		object bar=3
	}
}
```

----

Converting JiK back to JSON is a trivial process: literal nodes are encoded as their literal value; array nodes are encoded as their items, comma-separated and surrounded with `[]`; object nodes are encoded as their key/value pairs, comma-separated and surrounded with `{}`.

Only valid JiK nodes can be encoded to JSON; if a JiK document contains an invalid node, the entire document must fail to encode, rather than "guessing" at the intent. As well, a JiK document must contain only a single top-level node to be valid, unless the output is intended to be a JSON stream, in which case arbitrary numbers of nodes are allowed, each a separate JSON value.

* A literal node is valid if it contains a single unnamed argument (or, if a child of an object node, a single unnamed string argument followed by a single unnamed argument).

* An array node is valid if it contains no named properties. If it's the child of an object node, it must contain, as its first argument, a string literal.

* An object node is valid if it contains no unnamed arguments, with the exception that if it's the child of an object node, it must contain, as its first argument, a string literal. It must not contain any repeated "key" strings among its children, whether expressed as named properties or child nodes.