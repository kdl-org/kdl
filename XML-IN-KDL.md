XML-in-KDL (XiK)
================

This specification describes a canonical way to losslessly encode XML in [KDL](https://kdl.dev). While this isn't a very useful thing to want to do on its own, it's occasionally useful when using a KDL toolchain while speaking with an XML-consuming or -emitting service.

This is version 1.0.0 of XiK.

XML-in-KDL (XiK from now on) is a KDL microsyntax for losslessly encoding XML into a KDL document. XML and KDL, luckily, have *very similar* data models (KDL is *almost* a superset of XML), so it's quite straightforward to encode most XML documents into KDL.

See [the website example](blob/main/examples/website.kdl) for an example of this grammar in use to encode an HTML document.

XML has several types of nodes, corresponding to certain KDL constructs:

* Elements, which have an element name, zero or more attribute, and zero or more children. These are encoded directly as KDL nodes, using the nodename, properties, and children nodes.
* Raw text. In "pure" XML dialects, where raw text only appears as the sole child of an element (never mixed with other elements as siblings), this is generally encoded as a final string argument in a KDL node; in "mixed" XML dialects, it can be encoded as a special KDL node with the name `-`.
* Comments are encoded as KDL block comments. (Or as an actual node type, for some use-cases.)
* Processing Instructions. These are encoded similarly to elements if their contents are sufficiently structured, with a `?` in front of their node name. If they're not sufficiently structured, their contents are just strings.
* Doctypes. These are encoded like unstructured PIs, just with the node name `!doctype`.

----

XML elements and KDL nodes have a direct correspondence. In XiK, an XML element is encoded in KDL by:

* making the element name the KDL node name
* making the attributes into KDL properties
* making the child nodes as KDL child nodes

For example, the XML `<element foo="bar"><child baz="qux" /></element>` is encoded into XiK as `element foo="bar" { child baz="qux" }`.

XML namespaces are encoded the same as XML: the node name simply contains a `:` character. Note that KDL identifier syntax allows `:` directly in an ident, so a name like `xml:space` or `xlink:href` is a valid node or property name.

----

Raw text contents of an element can be encoded in two possible ways.

If the element contains *only* text, it should be encoded as a final string unnamed argument. For example, the XML `<a href="http://example.com">here's a link</a>` can be encoded as `a href="http://example.com" "here's a link"`.

If the element contains mixed text and element children, the text can be encoded as a KDL node with the name `-` with a single string unnamed argument. For example, the XML `<span>some <b>bold</b> text</span>` can be encoded as `span { - "some "; b "bold"; - " text" }`.

An element that contains only text *is allowed to* encode it as `-` children. For example, `<span>foo</span>` *may* be encoded as `span { - "foo" }` instead of `span "foo"`. However, an element cannot mix the "final string attribute" with child nodes; `span "foo" { b "bar" }` is an **invalid** encoding of `<span>foo<b>bar</b></span>`. (It must be encoded as `span { - "foo"; b "bar" }`.)

CDATA sections are not preserved in this encoding, as they are merely a source convenience so you don't have to escape a bunch of characters. They are encoded as normal textual contents would be.

-----

Comments are encoded as KDL multiline comments.  For example, `<!-- comment! -->` is encoded as `/* comment! */`.

If you are using a KDL toolchain that discards comments, and you *specifically* want to reflect the comment into XML, comments can be encoded as a special node name `!`, with a single unnamed string argument containing the comment's value (everything between the `<!--` and the `-->`). For example, `<!-- comment! -->` is encoded as the node `! " comment! "`.

----

Processing instructions and XML declarations (nodes that look like `<?foo ... ?>`) are encoded as nodes, with their name being the PI name preceded by a `?`. For example, an XML declaration (written like `<?xml ... ?>`) has the node name `?xml`.

The contents of a PI are technically completely unstructured. However, in practice most PIs' contents look like start-tag attributes. If this is the case, they should be encoded as properties on the node, with string values. For example, `<?xml version="1.0"?>` is encoded as `?xml version="1.0"`.

If the contents of a PI do *not* look like attributes, then instead the entire contents (from the end of the whitespace following the PI name, to the closing `?>` characters) are encoded as a single unnamed string value. For example, the preceding XML declaration *could* be alternately encoded as `?xml r#"version="1.0""#` (but shouldn't be).

(Note that XML declarations are not needed when writing XiK directly; the version is always 1.0, and the encoding is always UTF-8 since it's KDL.)

----

Doctypes (nodes that look like `<!DOCTYPE ...>`) are encoded similarly to unstructured Processing Instructions. They have a node name of `!doctype`, and the entire contents of the node, from the end of the whitespace following the "DOCTYPE" to the closing `>`, are encoded as a single unnamed string value. For example, the HTML doctype `<!DOCTYPE html>` is encoded as `!doctype "html"`, while the XHTML 1 Strict doctype would be encoded as `!doctype r#"html PUBLIC "-//W3C//DTD XHTML 1.0 Strict//EN" "http://www.w3.org/TR/xhtml1/DTD/xhtml1-strict.dtd""#`

----

Converting XiK back to XML is a trivial process:

* Element nodes are emitted as XML start tags, with the appropriate element name and attributes, followed by their contents emitted in order, followed by the appropriate end tag. If there are no contents, they should be emitted as a self-closing tag.
* Raw text is escaped appropriately when emitted. At the converter's discretion, CDATA segments can be used to encode any segment of raw text, as they deem fit. (This can be heuristic, based on the density of escapes required; or specialized to an output language, like always encoding the contents of HTML `script` and `style` elements with CDATA; or via any other criteria.)
* Comments are emitted as their contents (if a KDL comment) or their unnamed string value (if a `!` node) surrounded by `<!--` and `-->`, escaped as appropriate.
* PIs are emitted as a `<` followed by their node name, then a space, then either their attributes escaped as appropriate (if "structured") or the contents of their string value (if "unstructured"), and finally a `?>`.
* Doctypes are emitted as `<!DOCTYPE `, followed by the contents of their string value escaped as appropriate, and finally a `>`.

Only valid XiK nodes can be encoded to XML; if a XiK document contains an invalid node, the entire document must fail to encode, rather than "guessing" at the intent. A XiK node is valid if the XML element it represents is well-formed, and it has the correct KDL structure:

* Element nodes must contain any number of properties with string values, and either a single unnamed string argument as its final value, *or* any number of child nodes.
* Comment nodes must contain a single unnamed string argument and nothing else.
* "Structured" PI nodes must contain any number of properties with string values, and nothing else. "Unstructured" PI nodes must contain a single unnamed string argument and nothing else.
* Doctype nodes must contain a single unnamed string argument and nothing else.

The XiK document must also represent a well-formed XML document in its overall structure - for example, it can only contain a single top-level element node, all namespaces must be declared before they are used, etc.
