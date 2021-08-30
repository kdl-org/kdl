# KDL Schema Specification

## Example

```kdl
document description="KDL Schema KDL schema in KDL" schema-url="https://github.com/zkat/kdl" {
    node "document" {
        prop "schema-url" type="url"
        prop "description" type="string"
        children id="node-children" {
            node "node" id="node" {
                value description="name of the node" type="string"
                prop "description" type="string"
                prop "id" type="string"
                prop "ref" type="string"
                children {
                    node "prop" description="node property key/value pair" {
                        prop "id" type="string"
                        prop "ref" type="string"
                        value description="property key" type="string"
                        prop "type" type="string"
                        prop "description" type="string"
                    }
                    node "value" description="one or more direct node values" {
                        prop "id" type="string"
                        prop "ref" type="string"
                        prop "description" type="string"
                        prop "type" type="string"
                    }
                    node "children" {
                        prop "id" type="string"
                        prop "ref" type="string"
                        prop "description" type="string"
                        children ref="node-children"
                    }
                }
            }
        }
    }
}
```
