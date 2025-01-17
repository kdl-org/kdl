# KDL Schema Specification

The KDL Schema specification describes a schema language for use with KDL,
written in KDL itself. A schema language allows users to describe and
constrain the allowed semantics of a KDL document. This can be used for many
purposes: documentation for users, automated verification, or even automated
generation of bindings!

This document describes KDL Schema version `2.0.0`. It is unreleased.

## The Formal Schema

For the full KDL Schema schema itself, see
[examples/kdl-schema.kdl](./examples/kdl-schema.kdl).

## Definition

### `document` node

This is the toplevel node in a KDL Schema. It is required, and there must be
exactly one, at the very toplevel of a document.

#### Values

None.

#### Properties

None.

#### Children

* [`info`](#info-node) - one info node for that describes the schema itself.
* [`node`](#node-node) - zero or more toplevel nodes for the KDL document this schema describes.
* [`definitions`](#definitions-node) (optional): Definitions of nodes, values, props, and children block to reference in the toplevel nodes.
* `node-names` (optional): [Validations](#validation-nodes) to apply to the _names_ of child nodes.
* `other-nodes-allowed` (optional): Whether to allow nodes other than the ones explicitly listed here. Defaults to `#false`.
* [`tag`](#tag-node) - zero or more toplevel tags for nodes in the KDL document that this schema describes.
* `tag-names` (optional): [Validations](#validation-nodes) to apply to the _names_ of tags of child nodes.
* `other-tags-allowed` (optional): Whether to allow node tags other than the ones explicitly listed here. Defaults to `#false`.

#### Example

```kdl
document {
    
}
```

### `info` node

The `info` node describes the schema itself.

#### Values

None.

#### Properties

None.

#### Children

* [`title`](#title-node) (optional): zero or more titles
* [`description`](#description-node) (optional): zero or more descriptions
* [`author`](#author-and-contributor-nodes) (optional): zero or more authors
* [`contributor`](#author-and-contributor-nodes) (optional): zero or more contributors
* [`link`](#link-node) (optional): zero or more URLs
* [`license`](#license-node) (optional): zero or more licenses
* [`published`](#published-and-modified-nodes) (optional): a publication date
* [`modified`](#published-and-modified-nodes) (optional): a modification date
* [`version`](#version-node) (optional): a [SemVer](https://semver.org/) version number

### `title` node

The title of the schema or the format it describes.

#### Values

* Title

#### Properties

* `lang` (optional): An IETF BCP 47 language tag

### `description` node

A description of the schema or the format it describes.

#### Values

* Description

#### Properties

* `lang` (optional): An IETF BCP 47 language tag

### `author` and `contributor` nodes

Author(s) of the schema.

#### Values

* Author name

#### Properties

* `orcid` (optional): The [ORCID](https://orcid.org/) of the author.

#### Children

* [`link`](#link-node) (optional): zero or more URLs

### `link` node

Links to the schema itself, and to sources about the schema.

#### Values

* URI/IRI - A URI/IRI that the link points to

#### Properties

* `rel`: what the link is for (`self` or `documentation`)
* `lang` (optional): An IETF BCP 47 language tag

### `license` node

The license(s) that the schema is licensed under.

#### Values

* License name - Name of the used license

#### Properties

* `spdx` (optional): an [SPDX license identifier](https://spdx.dev/ids/)

#### Children

* [`link`](#link-node): one or more URLs

### `published` and `modified` nodes

When the schema was published or last modified respectively.

#### Values

* Publication or modification date - As a ISO8601 date

#### Properties

* `time` (optional): an ISO8601 Time to accompany the date

### `version` nodes

The version number of this version of the schema.

#### Values

* Version - Semver version specification

### `node` node

The `node` node describes node instances in a document. These may either be at
the toplevel of the document, or they may be nested inside a children block in
another node.

#### Values

* Node name (optional) - A string name for the node. If present, the node's rules/validations will apply only to children with this node name. Otherwise, the rules will apply to _all_ child nodes, regardless of whether they're named or not.

#### Properties

* `description` (optional): An informational description of the purpose of this node.
* `id` (optional): A globally unique identifier for this node.
* `ref` (optional): A [KDL Query](./QUERY-SPEC.md) string relative to the root of the document. If present, all properties, values, and children defined in the target node will be copied to this node, replacing any conflicts.

#### Children

* `min` (optional): Minimum number of this kind of node (or any node, if the name is missing) allowed in the parent's children block.
* `max` (optional): Maximum number of this kind of node (or any node, if the name is missing) allowed in the parent's children block.
* `prop-names` (optional): [Validations](#validation-nodes) to apply to the _names_ of properties.
* `other-props-allowed` (optional): Whether to allow props other than the ones explicitly listed here. Defaults to `false`.
* `tag`: [Validations](#validation-nodes) to apply to the tag of the node.
* [`prop`](#prop-node) - zero or more properties for this node.
* [`value`](#value-node) - zero or more values for this node.
* [`children`](#children-node) - zero or more children for this node.

### `tag` node

The `tag` describes the tags allowed in a children block or toplevel document.

#### Values

* Tag name (optional) - A tag for the node. If present, the node's rules/validations will apply only to children with this tag. Otherwise, the rules will apply to _all_ child nodes with tags.

#### Properties

* `description` (optional): An informational description of the purpose of this node.
* `id` (optional): A globally unique identifier for this node.
* `ref` (optional): A [KDL Query](./QUERY-SPEC.md) string relative to the root of the document. If present, all properties, values, and children defined in the target node will be copied to this node, replacing any conflicts.

#### Children

* [`node`](#node-node) - zero or more toplevel nodes that this tag is allowed to be on.
* `node-names` (optional): [Validations](#validation-nodes) to apply to the _names_ of nodes with this tag.
* `other-nodes-allowed` (optional): Whether to allow nodes other than the ones explicitly listed here. Defaults to `false`.

### `prop` node

Represents a property of a node, which is a key/value pair in KDL.

#### Values

* `key` (optional) - String key for the property. If this value is missing, the `prop` node's attributes will apply to all properties of its parent.

#### Properties

* `description` (optional): An informational description of the purpose of this property.
* `id` (optional): A globally unique identifier for this property.
* `ref` (optional): A [KDL Query](./QUERY-SPEC.md) string relative to the root of the document. If present, all properties defined in the target property will be copied to this property, replacing any conflicts.

#### Children

* `required` (optional): A boolean value indicating whether this property is required.
* Any [validation node](#validation-nodes).

### `value` node

Used to describe one or more values for a KDL node.

#### Values

None.

#### Properties

* `description` (optional): An informational description of the purpose of this value.
* `id` (optional): A globally unique identifier for this value.
* `ref` (optional): A [KDL Query](./QUERY-SPEC.md) string relative to the root of the document. If present, all values defined in the target value will be copied to this value, replacing any conflicts.

#### Children

* `min` (optional): Minimum number of values allowed.
* `max` (optional): Maximum number of values allowed.
* Any [validation node](#validation-nodes).

### `children` node

Denotes KDL node children.

#### Values

None.

#### Properties

* `description` (optional): An informational description of the purpose of this children block.
* `id` (optional): A globally unique identifier for this children block.
* `ref` (optional): A [KDL Query](./QUERY-SPEC.md) string relative to the root of the document. If present, all children defined in the target children block will be copied to this children block, replacing any conflicts.

#### Children

* [`node`](#node-node) - zero or more child nodes.
* `node-names` (optional): [Validations](#validation-nodes) to apply to the _names_ of child nodes.
* `other-nodes-allowed` (optional): Whether to allow nodes other than the ones explicitly listed here. Defaults to `false`.

### Validation Nodes

The following nodes are shared validations between props and values, and can
be used as children to either definition. They are also used to verify node
and property names when the `node-names` or `prop-names` options are activated.

#### Generic validations

* `tag`: [Validations](#validation-nodes) to apply to the tag of the value.
* `type`: A string denoting the type of the property value.
* `enum`: A specific list of allowed values for this property. May be heterogenous as long as it agrees with the `type`, if specified.

#### String validations

* `pattern`: Regex pattern or patterns to test prop values against. Specific regex syntax may be implementation-dependent.
* `min-length`: Minimum length, if a string.
* `max-length`: Maximum length, if a string.
* `format`: Intended data format, if the value is a string. Reserved values are:
    * `date-time`: ISO8601 date/time format.
    * `time`: "Time" section of ISO8601.
    * `date`: "Date" section of ISO8601.
    * `duration`: ISO8601 duration format.
    * `decimal`: IEEE 754-2008 decimal string format.
    * `currency`: ISO 4217 currency code.
    * `country-2`: ISO 3166-1 alpha-2 country code.
    * `country-3`: ISO 3166-1 alpha-3 country code.
    * `country-subdivision`: ISO 3166-2 country subdivision code.
    * `email`: RFC5302 email address.
    * `idn-email`: RFC6531 internationalized email address.
    * `hostname`: RFC1123 internet hostname.
    * `idn-hostname`: RFC5890 internationalized internet hostname.
    * `ipv4`: RFC2673 dotted-quad IPv4 address.
    * `ipv6`: RFC2373 IPv6 address.
    * `url`: RFC3986 URI.
    * `url-reference`: RFC3986 URI Reference.
    * `irl`: RFC3987 Internationalized Resource Identifier.
    * `irl-reference`: RFC3987 Internationalized Resource Identifier Reference.
    * `url-template`: RFC6570 URI Template.
    * `uuid`: RFC4122 UUID.
    * `regex`: Regular expression. Specific patterns may be implementation-dependent.
    * `base64`: A Base64-encoded string, denoting arbitrary binary data.
    * `kdl-query`: A [KDL Query](./QUERY-SPEC.md) string.

#### Number validations

* `%`: Only used for numeric values. Constrains them to be multiples of the given number(s).
* `>`: Greater than.
* `>=`: Greater than or equal to.
* `<`: Less than.
* `<=`: Less than or equal to.
* `format`: Intended data format for numeric values. Reserved values are:
    * `i8`: 8-bit signed integer
    * `i16`: 16-bit signed integer
    * `i32`: 32-bit signed integer
    * `i64`: 64-bit signed integer
    * `i128`: 128-bit signed integer
    * `u8`: 8-bit unsigned integer
    * `u16`: 16-bit unsigned integer
    * `u32`: 32-bit unsigned integer
    * `u64`: 64-bit unsigned integer
    * `u128`: 128-bit unsigned integer
    * `isize`: Platform-dependent signed integer
    * `usize`: Platform-dependent unsigned integer
    * `f32`: IEEE 754 single (32-bit) precision floating point number
    * `f64`: IEEE 754 double (64-bit) precision floating point number
    * `decimal64`: IEEE 754-2008 64-bit decimal floating point number
    * `decimal128`: IEEE 754-2008 128-bit decimal floating point number

### `definitions` node

Definitions to reference in parts of the top-level `node`s.

#### Values

None.

#### Properties

None.

#### Children

* [`node`](#node-node) - zero or more node definitions.
* [`tag`](#tag-node) - zero or more toplevel tags for nodes in the KDL document that this schema describes.
* [`prop`](#prop-node) - zero or more property definitions.
* [`value`](#value-node) - zero or more value definitions.
* [`children`](#children-node) - zero or more definitions of children blocks.
