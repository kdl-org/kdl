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

A node consists of three mandatory and two optional elements:

* a **name** being a Unicode string
* an optional **tag** being a Unicode string
* a sequence of **arguments**, each being a [value](#value)
* a set of **properties**, each consisting of
    * a name being Unicode string unique within the set
    * a [value](#value)
* an optional list of **children** being a [document](#document)

### Value

A value is one of:

* a Unicode string
* a **number**, being an arbitrary-precision, base-10 decimal number value
* a **boolean**, being one of the special values *true* and *false*
* the special value *null*

## Application Notes

Implementations may want to limit the set of processable KDL documents by
limiting properties of the data model such as the following:

* A node with empty string tag (e.g. `("")node`) differs from a node without
  tag (`node`).

* A node with empty children (e.g. `node {}`) differs from a node without
  children (`node`).

* KDL does not differ between integer numbers and numbers with fractional part
  nor does it define a fixed limit to the length or precision of numbers.

* property names differ also if their distinct strings become equivalent after
  Unicode normalization.

Applications must make clear whether they support full KDL data model or a
specific subset and whether they modify KDL documents to fit to their limited
model. Applications may further extend their document model with additional
information such as line numbers beyond the scope of this specification.

