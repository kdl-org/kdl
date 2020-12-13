use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub name: String,
    pub values: Vec<NodeValue>,
    pub properties: HashMap<String, NodeValue>,
    pub children: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeValue {
    Int(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
}
