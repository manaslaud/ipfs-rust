use cid::Cid;
use crate::cid::generator::generate_cid;
use thiserror::Error;
use derive_more::Display;

// Custom error definitions
#[derive(Debug, Error)]

pub enum DagErrors {
    #[error("The vector of leaves was empty")]
    EmptyError,
}

#[derive(Debug, Clone)]
#[derive(Display)]
#[display(fmt = "({}, {:?},{:?})", cid,links,data)]
struct MerkleNode {
    cid: Cid,             // Hash of this node
    links: Vec<Cid>,      // Child nodes
    data: Option<Vec<u8>> // File data (only for leaves)
}

fn ensure_even(data: &mut Vec<MerkleNode>) {
    if data.len() % 2 != 0 {
        let last = data.last().unwrap().clone();
        data.push(last);
    }
}

fn generate_merkle_root(nodes: &mut Vec<MerkleNode>) -> Result<MerkleNode, DagErrors> {
    if nodes.is_empty() {
        return Err(DagErrors::EmptyError);
    }

    ensure_even(nodes);

    let mut combined_nodes = Vec::new();

    for chunk in nodes.chunks(2) {
        let combined_data: Vec<u8> = chunk.iter()
            .flat_map(|node| node.cid.to_bytes())
            .collect();
        
        let new_cid = generate_cid(&combined_data);

        combined_nodes.push(MerkleNode {
            cid: new_cid,
            links: chunk.iter().map(|node| node.cid.clone()).collect(),
            data: None, // Internal nodes don't store data
        });
    }

    if combined_nodes.len() == 1 {
        return Ok(combined_nodes[0].clone());
    }

    generate_merkle_root(&mut combined_nodes)
}

pub fn generate_merkle_tree(leaves: Vec<MerkleNode>) -> Result<Vec<MerkleNode>, DagErrors> {
    if leaves.is_empty() {
        return Err(DagErrors::EmptyError);
    }

    let mut tree: Vec<MerkleNode> = leaves.clone();

    let mut current_level = leaves;

    while current_level.len() > 1 {
        ensure_even(&mut current_level);

        let mut next_level = Vec::new();

        for chunk in current_level.chunks(2) {
            let combined_data: Vec<u8> = chunk.iter()
                .flat_map(|node| node.cid.to_bytes())
                .collect();

            let new_cid = generate_cid(&combined_data);

            next_level.push(MerkleNode {
                cid: new_cid,
                links: chunk.iter().map(|node| node.cid.clone()).collect(),
                data: None,
            });
        }

        tree.extend(next_level.clone());
        current_level = next_level;
    }

    Ok(tree)
}

#[cfg(test)]
mod tests {
    use super::*;
    // use multihash::Multihash;

    fn create_leaf(data: &[u8]) -> MerkleNode {
        MerkleNode {
            cid: generate_cid(data),
            links: vec![],
            data: Some(data.to_vec()),
        }
    }

    #[test]
    fn test_generate_merkle_tree() {
        let leaves = vec![
            create_leaf(b"File Chunk 1"),
            create_leaf(b"File Chunk 2"),
            create_leaf(b"File Chunk 3"),
            create_leaf(b"File Chunk 4"),
        ];

        let tree = generate_merkle_tree(leaves.clone()).unwrap();
        for x in 0..tree.len() {
            println!("{}",tree[x]);
        }
        assert!(!tree.is_empty());
        assert_eq!(tree.last().unwrap().links.len(), 2); // Root should have 2 children
    }
}
