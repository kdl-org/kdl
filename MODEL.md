# KDL Data Model

This document specifies an abstract data model of KDL document language. 

*This is version `0.0.0` of KDL Data model. It has not been released yet.*

## Introduction

KDL is defined by [KDL Specification](SPEC.md) as a formal language with
components such as nodes, identifiers, strings, comments, and whitespace.
Some of these components can be expressed in mutliple ways and some of
these components are typically ignored when a KDL document is parsed.

KDL Data model defines a conceptual structure of semantically relevant elements
expressed in KDL syntax. The data model is required to process KDL documents
independent from syntax and implementations with [KDL Query
Language](QUERY-SPEC.md) and [KDL Schema Language](SCHEMA-SPEC.md).

## Elements

Every KDL document represents a [document](#document).

Sequences and sets can be empty.

A Unicode string is a sequence of Unicode code points.

### Document

A document is a sequence of [nodes](#node).

Documents must not contain itself via node direct or indirect children to avoid
circular structures.

### Node

A node consists of five elements:

* a **name** being a Unicode string
* a **tag** being a Unicode string
* a sequence of **arguments**, each being a [value](#value)
* a set of **properties**, each consisting of
    * a name being Unicode string unique within the set
    * a [value](#value)
* a list of **children** being a [document](#document)

### Value

A value consists of two elements:

* a **tag** being a Unicode string

and one one of:

* a Unicode string
* a **number**, being an arbitrary-precision, base-10 decimal number value
* a **boolean**, being one of the special values *true* and *false*
* the special value *null*

## Implementation Notes

### Extensions to the data model

While valid implementations must support at least the elements described above,
they *may* recognize and preserve additional information not captured in the
data model, such as:

* Line numbers and character position of parsed KDL syntax elements.

* Comments and precise details of whitespace and node terminators.

* Whether a tag is the empty string (`("")node)` or missing. KDL syntax allows
  nodes and values with and without tag. Both are identical in KDL data model.

* Whether a node had an empty child list (`node {}`) or no child list at all.
  KDL syntax allows both. KDL data model considers these identical.
 
* The precise format of numbers, such as what radix they're specified in
  (`0x1a`), whether they are an integer or not (`1` vs `1.0`), the presence of
  underscores (`1_234`), etc. KDL syntax supports multiple ways to specify
  numbers. KDL data model does not differentiate number types.

### Data binding

The mapping of KDL elements to data elements of a particular programming or
database languages is beyond the scope of this data model. Implementations
should use tags as type annotations to map KDL data model instances to other
type systems.

### Limitations to the data model

Implementations may choose to limit the set of processable KDL documents for
technical reasons. Such limitations must be stated clearly to indicate a useful
but incomplete support of KDL data model. Reasonable limitations include:

* Precision of numbers

* Types of elements that can have tags (e.g. disallow tags for boolean values
  and `null`)

* Unicode Normalization (e.g. collapse properties into one when their names are
  equivalent after normalization)

Implementations must document how limitations to KDL model are applied when KDL
document are read (e.g. give warnings and ignore unsupported elements).
