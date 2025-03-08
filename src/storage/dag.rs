use cid::Cid;
use serde::{Deserialize, Serialize};
use crate::cid::generator::generate_cid;
use thiserror::Error;
use derive_more::Display;
use crate::helpers::helper::convert_file_extension_to_raw;
// Custom error definitions
#[derive(Debug, Error)]
pub enum DagErrors {
    #[error("The vector of leaves was empty")]
    EmptyError,
    #[error("The file extension is invalid")]
    InvalidFileExtensionError
}

#[derive(Debug, Clone)]
#[derive(Display)]
#[derive(Serialize,Deserialize)]
#[display(fmt = "({:?} \n)",data)]
#[derive(PartialEq)]
pub struct MerkleNode {
    pub cid: Cid,             // Hash of this node
    pub links: Vec<Cid>,      // Child nodes
    pub data: Option<Vec<u8>>, // File data (only for leaves)
    pub is_dup:bool
}


pub fn generate_merkle_tree(
    leaves: Vec<MerkleNode>,
    file_extension: &str,
) -> Result<Vec<MerkleNode>, DagErrors> {
    if leaves.is_empty() {
        return Err(DagErrors::EmptyError);
    }
    if file_extension.is_empty() {
        return Err(DagErrors::InvalidFileExtensionError);
    }

    // Start with the initial leaves in the tree.
    let mut tree: Vec<MerkleNode> = leaves.clone();
    let mut current_level: Vec<MerkleNode> = leaves;

    while current_level.len() > 1 {
        let mut next_level: Vec<MerkleNode> = Vec::new();
        let mut i = 0;

        while i < current_level.len() {
            if i + 1 < current_level.len() {
                // Standard pairing: two nodes available.
                let left = &current_level[i];
                let right = &current_level[i + 1];

                let combined_data: Vec<u8> = left.cid.to_bytes()
                    .into_iter()
                    .chain(right.cid.to_bytes())
                    .collect();
                let new_cid = generate_cid(&combined_data);

                next_level.push(MerkleNode {
                    cid: new_cid,
                    links: vec![left.cid.clone(), right.cid.clone()],
                    data: None,
                    is_dup: false,
                });
                i += 2;
            } else {
                // Odd node: hash the node with itself.
                let node = &current_level[i];

                let combined_data: Vec<u8> = node.cid.to_bytes()
                    .into_iter()
                    .chain(node.cid.to_bytes())
                    .collect();
                let new_cid = generate_cid(&combined_data);

                next_level.push(MerkleNode {
                    cid: new_cid,
                    links: vec![node.cid.clone()],
                    data: None,
                    is_dup: false,
                });
                i += 1;
            }
        }

        // Add the newly created parent nodes to the overall tree.
        tree.extend(next_level.clone());
        current_level = next_level;
    }

    // Attach file_extension data to the root node.
    if let Some(last_node) = tree.last_mut() {
        last_node.data = convert_file_extension_to_raw(file_extension);
    }
    Ok(tree)
}

pub fn create_leaf(data: &[u8]) -> MerkleNode {
    MerkleNode {
        cid: generate_cid(data),
        links: vec![],
        data: Some(data.to_vec()),
        is_dup:false
    }
}
#[cfg(test)]
mod tests {
    use crate::helpers::helper::convert_raw_to_file_extension;

    use super::*;
    // use multihash::Multihash;

    #[test]
    fn test_generate_merkle_tree() {
        let leaves = vec![
            create_leaf(b"File Chunk 12"),
            create_leaf(b"File Chunk 22"),
            create_leaf(b"File Chunk 32"),
            create_leaf(b"File Chunk 42"),
            create_leaf(b"File Chunk 42"),
        ];

        let tree = generate_merkle_tree(leaves.clone(),"png").unwrap();
        for x in 0..tree.len() {
            println!("{:?}",tree[x].links);
        }
        assert!(!tree.is_empty());
        assert_eq!(
            convert_raw_to_file_extension(tree.last().unwrap().data.as_ref().unwrap().clone()).unwrap(),
            "png".to_string()
        );
    }
}
