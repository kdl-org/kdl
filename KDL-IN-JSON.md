# KDL-in-JSON (KiJ)

This specification describes a canonical way to encode KDL in JSON. This may be useful for describing KDL documents in a language that doesn't support custom data structures, but has the ability to represent JSON.

This is version 1.0.0 of KiJ. This specification defines a formal [JSON schema](https://json-schema.org/) as [schema-v1.json](./schema-v1.json).

## Scope

This specification allows to store either KDL v1 or KDL v2 documents in the JSON format. This may be used to convert any KDL v1 document to v2, but not always backwards (see [Compatibility](#Compatibility)).

The stored JSON is not required to be a minimal representation of a specific KDL document. The result is only guaranteed to keep all semantically important data from the KDL document:

* Whitespace (including comments) is fully omitted.
* Information about the string format (multiline in v2, raw in v1/v2) is lost, but string data is kept as-is.

In this document, JSON properties in an object are often referred to as "fields".

## Optional fields

Many fields in the schema are optional. Programming languages often use `null` values (or their analogue like `nil` or `None`) as both "absence of a value" and "empty value". To simplify the implementation of the schema converters, many situations are considered equivalent:

* For JSON arrays: absence of a field, `null` and empty array.
* For JSON objects: absence of a field, `null` and empty object.
* For JSON booleans: absence of a field, `null` and `false`.
* For JSON strings: absence of a field and `null`, but not an empty string.

Therefore, from now on, an "optional field" in context of these JSON data types can be one of these values.

## Values

Values correspond to a single KDL value, excluding type annotation. Values with a type annotation are called [Entries](#Entry).

Values are a JSON object with two possible properties:

* `data` is a required string that stores the data of the value.
* `literal` is an optional boolean that tells whether data belongs to a literal.

Empty value object is not valid, because `data` is required.

### Strings

By default `literal` is `false`. In this scenario, `data` is the data contained in a KDL string. Example:

```json
{ "data": "ident" }
```

This value can be represented in any way in the source KDL document. For example, all arguments in the node `node` are the same for the JSON schema:

```kdl
/- kdl-version 2
node ident "ident" #"ident"# "ident\  " """
	ident
"""
```

### Literals

If `literal` is `true`, then `data` is either a keyword or a number. These values don't overlap, so it's unambigious. Examples:

```json
[
	{ "literal": true, "data": "true" },
	{ "literal": true, "data": "false" },
	{ "literal": true, "data": "null" },
	{ "literal": true, "data": "nan" },
	{ "literal": true, "data": "inf" },
	{ "literal": true, "data": "-inf" },
	{ "literal": true, "data": "1" },
	{ "literal": true, "data": "0b1010" },
	{ "literal": true, "data": "-5.2e5" }
]
```

`true`, `false` and `null` map to the corresponding keywords in both KDL v1 and v2. `nan`, `inf` and `-inf`, however, are not available in the KDL v1 specification, so they are limited to v2 only. These values are case-sensitive.

If the `data` doesn't match any of the keywords, then the value is a number. It follows all the rules of the KDL grammar for the numbers (it is the same for KDL v1 and v2).

The numbers can be either preserved in the original form (with underscores, base prefix, exponent) or translated to another, as long as the value doesn't change. For example, `1.0` can be stored as `1` or `0b10` can be stored as `3`.

## Entry

Entries are KDL values with the type annotation. They are represented as a JSON object with two possible properties:

* `type` is an optional string with the type annotation.
* `value` is a required value of this entry.

`type` follows the same rules as the `data` field in the [string value](#Strings). `value` has the earlier defined [value object](#Values).

Empty entry object is not valid, because `value` is required.

## Node

Nodes are KDL nodes. They are represented as a JSON object with 5 possible properties:

* `type` is an optional string with the type annotation.
* `name` is a required string with the node name.
* `args` is an optional array of node arguments.
* `props` is an optional object of node properties.
* `nodes` is an optional array of children nodes.

`type` and `name` follow the same rules as the `data` field in the [string value](#Strings). However, `type` is optional, while `name` is not.

`args` is an array of [entries](#Entry), and `nodes` is an array of children [nodes](#Node). Order of the elements is important.

`props` accepts arbitraty JSON strings as the properties (fields, keys), and they are mapped to property keys. They follow the same rules as `name` or `type` in a node. The order of properties in KDL is not important, as well as in JSON.

Empty node object is not valid, because `name` is required.

## Document

Document is a KDL document, and in this schema it is represented as an optional array of nodes. It has the same semantics as the `nodes` field in a [node](#Node). This means that `null` or empty array is a valid (and empty) KDL document.

## Compatibility

As mentioned in the section about [literals](#Literals), this schema is not fully compatible for converting both KDL v1 and KDL v2 back and forth.

KDL v1 doesn't have keywords for IEEE754 floats like NaN and infinities, so a schema with these keywords is considered for KDL v2 only.

However, all KDL v1 documents are representable as this schema, and this can be used as an automatic way to convert documents to v2, if comments and whitespace is not important.

## Example

Here is an example of a JSON that stores the KDL example on the website:

```json
[
  {
    "name": "package",
    "nodes": [
      { "name": "name", "args": [{ "value": { "data": "my-pkg" } }] },
      { "name": "version", "args": [{ "value": { "data": "1.2.3" } }] },
      {
        "name": "dependencies",
        "nodes": [
          {
            "name": "lodash",
            "args": [{ "value": { "data": "^3.2.1" } }],
            "props": {
              "alias": { "value": { "data": "underscore" } },
              "optional": { "value": { "data": "true", "literal": true } }
            }
          }
        ]
      },
      {
        "name": "scripts",
        "nodes": [
          {
            "name": "message",
            "args": [{ "value": { "data": "hello\nworld" } }]
          },
          {
            "name": "build",
            "args": [
              {
                "value": {
                  "data": "echo \"foo\"\nnode -c \"console.log('hello, world!');\"\necho \"foo\" > some-file.txt"
                }
              }
            ]
          }
        ]
      },
      {
        "name": "the-matrix",
        "args": [
          { "value": { "data": "1", "literal": true } },
          { "value": { "data": "2", "literal": true } },
          { "value": { "data": "3", "literal": true } },
          { "value": { "data": "4", "literal": true } },
          { "value": { "data": "5", "literal": true } },
          { "value": { "data": "6", "literal": true } },
          { "value": { "data": "7", "literal": true } },
          { "value": { "data": "8", "literal": true } },
          { "value": { "data": "9", "literal": true } }
        ]
      }
    ]
  }
]
```
