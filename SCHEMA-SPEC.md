# KDL Schema Specification

The KDL Schema specification describes a schema language for use with KDL,
written in KDL itself. A schema language allows users to describe and
constrain the allowed semantics of a KDL document. This can be used for many
purposes: documentation for users, automated verification, or even automated
generation of bindings!

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
* [`prop`](#prop-node) - zero or more properties for this node.
* [`value`](#value-node) - zero or more values for this node.
* [`children`](#children-node) - zero or more children for this node.

### `prop` node
### `value` node
### `children` node

## The Schema

For the full KDL Schema schema itself, see
[examples/kdl-schema.kdl](./examples/kdl-schema.kdl).
