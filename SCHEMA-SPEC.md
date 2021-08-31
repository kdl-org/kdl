# KDL Schema Specification

The KDL Schema specification describes a schema language for use with KDL,
written in KDL itself. A schema language allows users to describe and
constrain the allowed semantics of a KDL document. This can be used for many
purposes: documentation for users, automated verification, or even automated
generation of bindings!

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

* `description` (optional): An informational description of the purpose of this schema.
* `schema-url` (optional): A URL where someone may go to find more information about this schema. It is not meant for mechanical processing.

#### Children

* [`node`](#node-node) - zero or more toplevel nodes for the KDL document this schema describes.
* `node-names` (optional): [Validations](#validation-nodes) to apply to the _names_ of child nodes.
* `other-nodes-allowed` (optional): Whether to allow nodes other than the ones explicitly listed here. Defaults to `false`.

### `node` node

The `node` node describes node instances in a document. These may either be at
the toplevel of the document, or they may be nested inside a children block in
another node.

#### Values

* Node name (optional) - A string name for the node. If present, the node's rules/validations will apply only to children with this node name. Otherwise, the rules will apply to _all_ child nodes, regardless of whether they're named or not.

#### Properties

* `description` (optional): An informational description of the purpose of this node.
* `id` (optional): A globally unique identifier for this node.
* `ref` (optional): A globally unique reference to another node's ID. If present, all properties, values, and children defined in the target node will be copied to this node, replacing any conflicts.

#### Children

* `min` (optional): Minimum number of this kind of node (or any node, if the name is missing) allowed in the parent's children block.
* `max` (optional): Maximum number of this kind of node (or any node, if the name is missing) allowed in the parent's children block.
* `prop-names` (optional): [Validations](#validation-nodes) to apply to the _names_ of properties.
* `other-props-allowed` (optional): Whether to allow props other than the ones explicitly listed here. Defaults to `false`.
* [`prop`](#prop-node) - zero or more properties for this node.
* [`value`](#value-node) - zero or more values for this node.
* [`children`](#children-node) - zero or more children for this node.

### `prop` node

Represents a property of a node, which is a key/value pair in KDL.

#### Values

* `key` (optional) - String key for the property. If this value is missing, the `prop` node's attributes will apply to all properties of its parent.

#### Properties

* `description` (optional): An informational description of the purpose of this property.
* `id` (optional): A globally unique identifier for this property.
* `ref` (optional): A globally unique reference to another property's ID. If present, all properties defined in the target property will be copied to this property, replacing any conflicts.

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
* `ref` (optional): A globally unique reference to another value's ID. If present, all values defined in the target value will be copied to this value, replacing any conflicts.

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
* `ref` (optional): A globally unique reference to another children block's ID. If present, all children defined in the target children block will be copied to this children block, replacing any conflicts.

#### Children

* [`node`](#node-node) - zero or more child nodes.
* `node-names` (optional): [Validations](#validation-nodes) to apply to the _names_ of child nodes.
* `other-nodes-allowed` (optional): Whether to allow nodes other than the ones explicitly listed here. Defaults to `false`.

### Validation Nodes

The following nodes are shared validations between props and values, and can
be used as children to either definition. They are also used to verify node
and property names when the `node-names` or `prop-names` options are activated.

#### Generic validations

* `type`: A string denoting the type of the property value.
* `enum`: A specific list of allowed values for this property. May be heterogenous as long as it agrees with the `type`, if specified.

#### String validations

* `pattern`: Regex pattern or patterns to test prop values against. Specific regex syntax may be implementation-dependent.
* `min-length`: Minimum length, if a string.
* `max-length`: Maximum length, if a string.
* `format`: Intended data format, if the value is a string. Possible values are:
    * `date-time`: ISO8601 date/time format.
    * `time`: "Time" section of ISO8601.
    * `date`: "Date" section of ISO8601.
    * `email`: RFC5302 email address.
    * `idn-email`: RFC6531 internationalized email address.
    * `hostname`: RFC1132 internet hostname.
    * `idn-hostname`: RFC5890 internationalized internet hostname.
    * `ipv4`: RFC2673 dotted-quad IPv4 address.
    * `ipv6`: RFC2373 IPv6 address.
    * `uri`: RFC3986 URI.
    * `uri-reference`: RFC3986 URI Reference.
    * `iri`: RFC3987 Internationalized Resource Identifier.
    * `iri-reference`: RFC3987 Internationalized Resource Identifier Reference.
    * `uri-template`: RFC6570 URI Template.
    * `uuid`: RFC4122 UUID.
    * `regex`: Regular expression. Specific patterns may be implementation-dependent.

#### Number validations

* `%`: Only used for numeric values. Constrains them to be multiples of the given number(s).
* `>`: Greater than.
* `>=`: Greater than or equal to.
* `<`: Less than.
* `<=`: Less than or equal to.
