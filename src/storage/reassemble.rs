use crate::storage::init_db;
use crate::storage::MerkleNode;
use cid::Cid;
use infer::get;
use queues::*;
use thiserror::Error;
// use serde::{Serialize, Deserialize};

const PATH: &str = "./tmp/data";

#[derive(Debug, Error)]
pub enum ReassembleErrors {
    #[error("The root node with this cid does not exist here.")]
    RootNodeNotFoundError,
    #[error("The node with this cid does not exist here.")]
    NotFoundError,
    #[error("Unknown Error Occured")]
    UnknownError,
}

pub fn detect_file_type(file_data: &[u8]) -> Option<String> {
    let kind = get(file_data);
    match kind {
        Some(val) => return Some(val.mime_type().to_string()),
        None => return None,
    }
}

/*
tldr; how it works
just fetching stuff from the db
*/
pub async fn return_node_from_db(cid_string: String) -> Option<MerkleNode> {
    let db = init_db(String::from(PATH)).await.unwrap();
    let root_db_node: Result<Option<fjall::Slice>, fjall::Error> = db.get(cid_string);

    let slice = match root_db_node {
        Ok(Some(value)) => value,
        Ok(None) => {
            eprintln!("{}", ReassembleErrors::RootNodeNotFoundError);
            return None;
        }
        Err(err) => {
            eprintln!(
                "Database error: {} - {}",
                err,
                ReassembleErrors::UnknownError
            );
            return None;
        }
    };

    let str_val = String::from_utf8_lossy(&slice).to_string();
    let node: MerkleNode = serde_json::from_str(&str_val).unwrap();
    Some(node)
}

/*
tldr; how it works
performing bfs works best i think here, because at every level, starting from the root node,
need to check for duplication of the last node (caused by the ensure_even) function
if last a dup node/ same as second last node, do not explore it's children
assemble all the data and in the end extract from the root node the extension type
1. check at each level if the last and second last node's cids are equal,
if so then no need to explore the last node, only explore second last
*/
pub async fn get_leaves_from_root_node_cid(
    cid_string: String,
) -> Result<Vec<MerkleNode>, ReassembleErrors> {
    let mut res: Vec<MerkleNode> = Vec::new();

    let root_node: Option<MerkleNode> = return_node_from_db(cid_string).await;
    if root_node.is_none() {
        return Err(ReassembleErrors::RootNodeNotFoundError);
    }
    // res.push(root_node.clone().unwrap());
    let mut q: Queue<Cid> = queue![];
    let root_node_links = root_node.unwrap().links;

    //added 2nd level links to the queue to explore from (l->r)
    for node_link in root_node_links {
        let _ = q.add(node_link);
    }
    while q.size() != 0 {
        //get the node whos link is q.remove()
        let node: Option<MerkleNode> = return_node_from_db(q.remove().unwrap().to_string()).await;
        if node.is_none() {
            return Err(ReassembleErrors::NotFoundError);
        }
        if let Some(node) = node {
            if !node.is_dup {
                let links = node.links.clone();
                if links.len() == 0 {
                    res.push(node);
                }
                for link in links {
                    let _ = q.add(link);
                }
            }
        }
    }
    if q.size() == 0 {
        return Ok(res);
    }
    return Err(ReassembleErrors::UnknownError);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::dag::{create_leaf, generate_merkle_tree};
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
            create_leaf(b"File Chunk 2"),

        ];

        let tree = generate_merkle_tree(leaves.clone(), "png").unwrap();
        println!("{}",tree.len());
        let root_node = tree.last().unwrap().cid.to_string();
        let res = store_file(tree).await;
        //check if the file is stored correctly
        assert_eq!(res, true);
        let retrived = return_node_from_db(root_node.to_string())
            .await
            .unwrap()
            .cid
            .to_string();
        assert_eq!(root_node, retrived);
    }

    #[tokio::test]
    async fn test_get_leaves_from_root_node_cid() {
        let leaves = vec![
            create_leaf(b"File Chunk 12"),
            create_leaf(b"File Chunk 22"),
            create_leaf(b"File Chunk 32"),
            create_leaf(b"File Chunk 42"),
            create_leaf(b"File Chunk 42"),

        ];
        let tree = generate_merkle_tree(leaves.clone(), "png").unwrap();
        let root_node = tree.last().unwrap().cid.to_string();
        let res = store_file(tree).await;
        assert_eq!(res, true);
        let retrieved_leaves=get_leaves_from_root_node_cid(root_node).await.unwrap();
        for x in retrieved_leaves {
            println!("{:?}",x.data);
        }
        // assert_eq!(retrieved_leaves,leaves);
    }
}
