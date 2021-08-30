# KDL Schema Specification

## Example

```kdl
document description="KDL Schema KDL schema in KDL" schema-url="https://github.com/zkat/kdl" {
    node "document" min=1 max=1 {
        prop "schema-url" type="url" \
            description="URL where you can find this schema. Informational only."
        prop "description" type="string" \
            description="General purpose and description for this document schema."
        children id="node-children" {
            node "node" id="node" {
                value min=1 max=1 type="string" \
                    description="The name of the node."
                prop "description" type="string" \
                    description="A description of this node's purpose."
                prop "id" type="string" \
                    description="globally-unique ID of this node."
                prop "ref" type="string" \
                    description="globally unique reference to this node."
                prop "min" type="number" \
                    description="minimum number of instances of this node in its parent's children."
                prop "max" type="number" \
                    description="maximum number of instances of this node in its parent's children."
                children {
                    node "prop" description="A node property key/value pair." {
                        value type="string" min=1 max=1 \
                            description="The property key."
                        prop "type" type="string" \
                            description="The type for this prop's value."
                        prop "id" type="string" \
                            description="A globally-unique ID of this property."
                        prop "ref" type="string" \
                            description="A globally unique reference to another property node."
                        prop "description" type="string" \
                            description="A description of this property's purpose."
                    }
                    node "value" description="one or more direct node values" {
                        prop "id" type="string" \
                            description="A globally-unique ID of this value."
                        prop "ref" type="string" \
                            description="A globally unique reference to another value node."
                        prop "type" type="string" \
                            description="The type for this value."
                        prop "description" type="string" \
                            description="A description of this property's purpose."
                        prop "min" type="number" \
                            description="Minimum number of this value to be provided to the node."
                        prop "max" type="number" \
                            description="Maximum number of this value to be provided to the node."
                    }
                    node "children" max=1 {
                        prop "id" type="string" \
                            description="A globally-unique ID of this children node."
                        prop "ref" type="string" \
                            description="A globally unique reference to another children node."
                        prop "description" type="string" \
                            description="A description of this property's purpose."
                        children ref="node-children"
                    }
                }
            }
        }
    }
}
```
