use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub values: Vec<NodeValue>,
    pub properties: HashMap<String, NodeValue>,
    pub children: Vec<Node>,
}

#[derive(Debug, Clone)]
pub enum NodeValue {}
