use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct KdlNode {
    pub name: String,
    pub values: Vec<KdlNodeValue>,
    pub properties: HashMap<String, KdlNodeValue>,
    pub children: Vec<KdlNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum KdlNodeValue {
    Int(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
}
