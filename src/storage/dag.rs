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

fn ensure_even(data: &mut Vec<MerkleNode>, tree: &mut Vec<MerkleNode>) {
    if data.len() % 2 != 0 {
        let mut last = data.last().unwrap().clone();
        last.is_dup = true;
        data.push(last.clone());
        // Record the duplicate node in the overall tree.
        tree.push(last);
    }
}


// fn generate_merkle_root(nodes: &mut Vec<MerkleNode>) -> Result<MerkleNode, DagErrors> {
//     if nodes.is_empty() {
//         return Err(DagErrors::EmptyError);
//     }

//     ensure_even(nodes);

//     let mut combined_nodes = Vec::new();

//     for chunk in nodes.chunks(2) {
//         let combined_data: Vec<u8> = chunk.iter()
//             .flat_map(|node| node.cid.to_bytes())
//             .collect();
        
//         let new_cid = generate_cid(&combined_data);

//         combined_nodes.push(MerkleNode {
//             cid: new_cid,
//             links: chunk.iter().map(|node| node.cid.clone()).collect(),
//             data: None, // Internal nodes don't store data
//             is_dup:false
//         });
//     }

//     if combined_nodes.len() == 1 {
//         return Ok(combined_nodes[0].clone());
//     }

//     generate_merkle_root(&mut combined_nodes)
// }

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
        // Ensure even number of nodes at this level and record any duplicate in the overall tree.
        ensure_even(&mut current_level, &mut tree);

        let mut next_level: Vec<MerkleNode> = Vec::new();

        for chunk in current_level.chunks(2) {
            let combined_data: Vec<u8> = chunk.iter()
                .flat_map(|node| node.cid.to_bytes())
                .collect();

            let new_cid = generate_cid(&combined_data);

            next_level.push(MerkleNode {
                cid: new_cid,
                links: chunk.iter().map(|node| node.cid.clone()).collect(),
                data: None,
                is_dup: false,
            });
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
            create_leaf(b"File Chunk 1"),
            create_leaf(b"File Chunk 2"),
            create_leaf(b"File Chunk 3"),
            create_leaf(b"File Chunk 4"),
        ];

        let tree = generate_merkle_tree(leaves.clone(),"png").unwrap();
        for x in 0..tree.len() {
            println!("{}",tree[x]);
        }
        assert!(!tree.is_empty());
        assert_eq!(
            convert_raw_to_file_extension(tree.last().unwrap().data.as_ref().unwrap().clone()).unwrap(),
            "png".to_string()
        );
                assert_eq!(tree.last().unwrap().links.len(), 2); // Root should have 2 children
    }
}
