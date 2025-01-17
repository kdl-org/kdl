# KDL Schema Language Specification

The KDL Schema Language specification describes a schema language for use with
KDL. A schema language allows users to describe and constrain the allowed
semantics of a document. This can be used for many purposes: documentation for
users, automated verification, or even automated generation of bindings!

This document describes KDL Schema version `2.0.0`. It is unreleased.

# The Formal Schema

For the full KDL Schema Language schema itself, see
[schema/kdl-schema.kdl](./schema/kdl-schema.kdl).

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

- **arg:** The title text.
  - **type:** `string`

#### `description`

**Schema description**

A description of the schema or the format it validates, which may include its purposes, its usage, and even examples.

- **arg:** Description text.
  - **type:** `string`

#### `author`

**Schema author**

An author for the schema.

- **repeatable**
- [Person](#person-mixin)

#### `contributor`

**Schema author**

A contributor to the schema, who might not be considered an author, per-se.

- **repeatable**
- [Person](#person-mixin)

#### `link`

A link related to this schema.

- **repeatable**
- [Link](#link-mixin)

#### `license`

**Schema license**

The license(s) that the schema is licensed under. At least one of `spdx`, `path`, or `url` props must be provided.

- **repeatable**
- **prop `spdx`:** An SPDX license identifier
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

Schema modification date

When the schema was modified. If used multiple times, the most recent date will be considered 'latest'.

- **repeatable**
- **arg:** Modification date
  - **type:** `string`
  - **format:** `date`, `date-time`

#### `version`

Schema semver version

The version number of this version of the schema, in semver format.

<hr />

## `example`

An example document validated by this schema

The `example` node is completely inert. It SHOULD contain an illustrative example of a document that would be valid if checked against this schema.

- **repeatable**
- [About](#about-mixin)

<hr />

<hr />

# Shared definitions

<a name="person-mixin"></a>

## Person

Shared definition for what makes a "person".

- **arg:** Person name
  - **type:** `string`
- **prop `orcid`:** The ORCID of the person
  - **type:** `string`

### Children

#### `link`

Link connected to this resource. Use `mailto:` for emails.

- **repeatable**
- [Link](#link-mixin)

<a name="link-mixin"></a>
## Link

External link

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

<a name="about-mixin"></a>
## About

**Description for this component**

By convention, the format of this value is intended to be similar to git's commit message system: The first line is treated as a short descriptor/summary, and any lines underneath it are treated as the longer-form documentation. As such, the first line SHOULD be up to 50 characters in length.

Tooling SHOULD only display some or all of the first line in user interfaces that call for terseness, and they SHOULD display both the short descriptor and the longer explanation when expanding it.

`about` can be provided either as a prop or a child.

- **prop `about`:**  Description for this component
  **type:** `string`

### Children

#### `about`

Description for this component

- **arg:** Description for this component
  - **required**
  - **type:** `string`
