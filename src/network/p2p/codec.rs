use crate::storage::dag::MerkleNode;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub enum ResponseType {
    Array(Vec<MerkleNode>),
    Single(MerkleNode),
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Response {
    block: MerkleNode,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub enum Depth {
    Full,
    Single,
}
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Request {
    cid: String,
    depth: Depth,
}
