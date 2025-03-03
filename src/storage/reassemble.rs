use infer::get;
use crate::storage::init_db;
use thiserror::Error;
use crate::storage::MerkleNode;
// use serde::{Serialize, Deserialize};

const PATH : &str = "./tmp/data";

#[derive(Debug, Error)]
pub enum ReassembleErrors {
    #[error("The root node with this cid does not exist here.")]
    RootNodeNotFoundError,
    #[error("The node with this cid does not exist here.")]
    NotFoundError,
    #[error("Unknown Error Occured")]
    UnknownError
} 

pub fn detect_file_type(file_data: &[u8]) -> Option<String> {
    let kind = get(file_data);
    match kind {
        Some(val)=>  return Some(val.mime_type().to_string()),
        None => return None
    }
}

/*
tldr; how it works
performing bfs works best i think here, because at every level, starting from the root node,
need to check for duplication of the last node (caused by the ensure_even) function
if last a dup node/ same as second last node, do not explore it's children
assemble all the data and in the end extract from the root node the extension type
*/
pub async fn return_node_from_db(cid_string:String) -> Option<MerkleNode>{
let db=init_db(String::from(PATH)).await.unwrap();
let root_db_node: Result<Option<fjall::Slice>, fjall::Error>=db.get(cid_string);

let slice = match root_db_node {
    Ok(Some(value)) => value, 
    Ok(None) => {
        eprintln!("{}", ReassembleErrors::RootNodeNotFoundError);   
        return None 
    }
    Err(err) => {
        eprintln!("Database error: {} - {}", err, ReassembleErrors::UnknownError);
        return None 
    }
};

let str_val=String::from_utf8_lossy(&slice).to_string();
let node:MerkleNode=serde_json::from_str(&str_val).unwrap();
Some(node)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::dag::{create_leaf,generate_merkle_tree};
    use crate::storage::store_file;

    #[test]
    fn test_detect_file_type() {
        let data: Vec<u8> = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        let kind = detect_file_type(&data).unwrap();
        assert_eq!(kind, "image/png");
    }
    #[tokio::test]
    async fn test_insertion_and_retrieval() {
        let leaves = vec![
            create_leaf(b"File Chunk 1"),
            create_leaf(b"File Chunk 2"),
            create_leaf(b"File Chunk 3"),
            create_leaf(b"File Chunk 4"),
        ];

        let tree = generate_merkle_tree(leaves.clone(),"png").unwrap();
        let root_node=tree.last().unwrap().cid.to_string();
        store_file(tree).await;
        let retrived=return_node_from_db(root_node.to_string()).await.unwrap().cid.to_string();
        assert_eq!(root_node,retrived);
    }
}
