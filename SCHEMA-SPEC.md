# KDL Schema Language Specification

The KDL Schema Language specification describes a schema language for use with
KDL. A schema language allows users to describe and constrain the allowed
semantics of a document. This can be used for many purposes: documentation for
users, automated verification, or even automated generation of bindings!

This document describes KDL Schema version `2.0.0`. It is unreleased.

# The Formal Schema

For the full KDL Schema Language schema itself, see
[schema/kdl-schema.kdl](./schema/kdl-schema.kdl).

## Referencing schemas

Schemas may be referenced directly in a KDL document by placing an inert `/- kdl-schema` node at the top of the file, before any other non-whitespace, non-comment content. Uncommented `kdl-schema` nodes, or nodes that are commented with `//` or `/* */` MUST be ignored, as they may be intended to be part of the document's actual data.

Multiple instances of this node can be present, and they will reference/name schemas that will be applied to the document. They MUST be properly-formatted URLs. Implementations MAY attempt to visit them, but MUST NOT assume they are valid.

If multiple `/- kdl-schema` nodes are present, ALL schemas MUST successfully validate in order for the document to validate, except for those for which `warn-only` is `#true`.

If validations pass except for `warn-only` ones, implementations SHOULD report which schema failed to pass—they SHOULD include more details about what the specific failure was, but MAY simply indicate that certain schema(s) failed to validate.

- **repeatable**
- **prop `warn-only`:** Validation failures should ONLY be warnings
  - **type:** `boolean`
  - **default:** `#false`
- **arg:** URL/IRL for this schema
  - **type:** `string`
  - **format:** `url`, `url-reference`, `irl`, `irl-reference`

# Definitions

There are four "toplevel" nodes in the KDL Schema Language. Each is defined below, along with the children they may have:

- [`metadata`](#metadata): General metadata about the schema itself
- [`example`](#example): An example document that's considered to be valid according to this schema
- [`definitions`](#definitions): An inert set of [shared node definitions](#shared-definitions) that may be references/mixed-in into the "true" definitions in the `document` node
- [`document`](#document): The main schema definition itself

## `metadata`

**Schema metadata**

Contains metadata about the schema itself.

- **required**

### Children

#### `id`

**Schema identifier**

The unique identifier for this schema. MUST be a valid URL/IRL. When parsing a schema, implementations SHOULD NOT attempt to visit the URL itself, as it is not necessary for it to be valid. Parsers verifying against a schema MAY look at the given URL for a document if they don't already have a valid copy.

- **arg:** URL/IRL identifier.
  - **type:** `string`
  - **format:** `url`, `irl`

#### `title`

**Schema title**

The title of the schema or the format it describes.

Multiple `description` nodes may be present, distinguished by a `lang` prop that can be used to specify the language or local of the text, using a two-letter ISO 639-1 language codes plus an optional hyphen (`-`) followed by an ISO 3166-1 country code, case insensitive.

- **arg:** The title text.
  - **type:** `string`
- **prop `lang`:** Locale/language code
  - **type:** `string`
  - **pattern:** `^[a-zA-Z]{2}(?:-[a-zA-Z]{2})?$`

#### `description`

**Schema description**

A description of the schema or the format it validates, which may include its purposes, its usage, and even examples.

Multiple `description` nodes may be present, distinguished by a `lang` prop that can be used to specify the language or local of the text, using a two-letter ISO 639-1 language codes plus an optional hyphen (`-`) followed by an ISO 3166-1 country code, case insensitive.

- **arg:** Description text.
  - **type:** `string`
- **prop `lang`:** Locale/language code
  - **type:** `string`
  - **pattern:** `^[a-zA-Z]{2}(?:-[a-zA-Z]{2})?$`

#### `author`

**Schema author**

An author for the schema.

- **repeatable**
- **ref:** [Person](#person-mixin)

#### `contributor`

**Schema author**

A contributor to the schema, who might not be considered an author, per-se.

- **repeatable**
- **ref:** [Person](#person-mixin)

#### `link`

**Schema link**

A link related to this schema.

- **repeatable**
- **ref:** [Link](#link-mixin)

#### `license`

**Schema license**

The license(s) that the schema is licensed under. At least one of `spdx`, `path`, or `url` props must be provided.

- **repeatable**
- **prop `spdx`:** An [SPDX](https://spdx.dev/) license identifier
  - **type:** `string`
- **prop `path`:** Path to a local license file. Relative paths MAY be interpreted in any way the program chooses, or MAY be ignored
  - **type:** `string`
- **prop `link`:** URL/IRL to an externally-stored license
  - **type:** `string`
  - **format:** `url`, `url-reference`, `irl`, `irl-reference`

#### `published`

**Schema publication date**

Date or datetime when the schema was published.

- **arg:** Publication date
  - **required**
  - **type:** `string`
  - **format:** `date`, `date-time`

#### `modified`

**Schema modification date**

When the schema was modified. If used multiple times, the most recent date will be considered 'latest'.

- **repeatable**
- **arg:** Modification date
  - **type:** `string`
  - **format:** `date`, `date-time`

#### `version`

**Schema semver version**

The version number of this version of the schema, in semver format. The pattern is validated against [the standard semver regular expression](https://semver.org/#is-there-a-suggested-regular-expression-regex-to-check-a-semver-string).

- **arg:** Semver version number
  - **type:** `string`
  - **pattern:** `^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$`

<hr />

## `example`

**Example document per this schema**

The `example` node is completely inert. It SHOULD contain an illustrative example of a document that would be valid if checked against this schema.

The [`about`](#about-mixin) prop or children can be used to describe what the example is about.

- **repeatable**
- **ref:** [About](#about-mixin)

<hr />

## `definitions`

**Inert validation definitions**

An optional set of definitions that may be referenced elsewhere in the schema. They will be inert (that is, not directly apply to the document) unless referenced by another node inside [`document`](#document) using the [`ref`](#ref) node.

<hr />

## `document`

**Validations for document contents**

This node is responsible for specifying active validations that will be applied to a document to check its conformance to a given schema. That is, this is the bulk of the definition of the schema.

### Children

#### `children`

**Node children**

Validations and definitions used for all nodes in this scope. Children are only allowed on nodes (or the toplevel document) if at least one `children` node is present in their definitions.

##### Children

###### `min`

**Minimum number of children**

- **arg:** Minimum number of children
  - **type:** `integer`
  - **default:** `0`

###### `max`

**Maximum number of children**

- **arg:** Maximum number of children
  - **type:** `integer`

###### `names`

**Child node name validations**

String validations to apply to all node names in this scope.

- **repeatable**
- **ref:** [String validations](#string-validations-mixin)

<hr />

# Shared definitions

<a name="person-mixin"></a>

## Person

Shared definition for what makes a "person".

- **arg:** Person name
  - **type:** `string`
- **prop `orcid`:** The ORCID of the person
  - **type:** `string`
- **ref:** [About](#about-mixin)

### Children

#### `link`

Link connected to this resource. Use `mailto:` for emails.

- **repeatable**
- **ref:** [Link](#link-mixin)

<a name="link-mixin"></a>

## Link

**External link**

Link to an external resource of some sort, such as the schema itself (`rel=self`) or documentation (`rel=documentation`). Implementations MAY visit the URL, but MUST NOT assume it is valid.

- **arg:** Link connected to this resource. Use `mailto:` for emails.
  - **type:** `string`
  - **format:** `url`, `irl`
- **prop `rel`:** The relationship between the current entity and the URL/IRL.
  - **type:** `string`
  - **default:** `self`
  - **enum:** (non-exhaustive)
    - `self`
    - `documentation`
    - `contact`
    - `organization`
- **ref:** [About](#about-mixin)

<a name="about-mixin"></a>

## About

**Description for this component**

By convention, the format of this value is intended to be similar to git's commit message system: The first line is treated as a short descriptor/summary, and any lines underneath it are treated as the longer-form documentation. As such, the first line SHOULD be up to 50 characters in length.

Tooling SHOULD only display some or all of the first line in user interfaces that call for terseness, and they SHOULD display both the short descriptor and the longer explanation when expanding it.

`about` can be provided either as a prop or a child.

Multiple `about` child nodes may also be used, and their `lang` properties can be used to specify the language or locale of the about text, using a two-letter ISO 639-1 language codes plus an optional hyphen (`-`) followed by an ISO 3166-1 country code, case insensitive.

- **prop `about`:** Description for this component
  **type:** `string`

### Children

#### `about`

Description for this component

Multiple `about` child nodes may also be used, and their `lang` properties can be used to specify the language or locale of the about text

- **repeatable**
- **ref:** [Lang](#lang-mixin)
- **arg:** Description for this component
  - **required**
  - **type:** `string`

<a name="lang-mixin"></a>

## Lang

**Language/locale code**

An ISO 639-1 language code plus by an optional hyphen (`-`) followed by an ISO 3166-1 country code, case insensitive.

Implementations MAY use the pattern OR use an `enum` to specify all possible codes.

- **type:** `string`
- **pattern:** `^[a-zA-Z]{2}(?:-[a-zA-Z]{2})?$`

<a name="shared-validations-mixin"></a>

## Shared validations

**Validations used by other validations**

### Children

#### `type`

**The type for this value**

Multiple arguments signify a sum type.

- **args:**
  - **min:** 1
  - **type:** `string`
  - **enum:** `string` `boolean` `number` `integer` `null`
  - **distinct**

#### `enum`

**Enumeration of values**

An enumeration of possible values.

The `allow-others` prop may be used to allow other choices to be present, as long as they pass other validations in the node.

While apparently redundant, this option may be useful in cases where there's a set of suggested values, but others are acceptable. This information can then be used by tooling to e.g. suggest completion items.

- **repeatable**
- **prop `allow-others`:** Allow other choices
  - **type:** `boolean`
  - **default:** `#false`
- **args:**
  - **min:** 1

##### Children

###### `-`

**Enumeration choice**

Dash children may be used when many enum values are present, or when there's value in including [about](#about-mixin) information about a value.

Dash children and node arguments are merged, in no particular order, with dash children being picked when duplicates are found.

- **arg:** Enum value
- **ref:** [About](#about-mixin)
