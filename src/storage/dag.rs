use serde::{Serialize, Deserialize};
use cid::Cid;
use crate::cid::generator::generate_cid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DagNode {
    pub cid: Cid,              
    pub data: Option<Vec<u8>>, 
    pub links: Vec<Cid>,       
}

pub fn chunk_file(data: &[u8], chunk_size: usize) -> Vec<DagNode> {
    let mut nodes: Vec<DagNode> = Vec::new();
    for chunk in data.chunks(chunk_size) {
        let cid: cid::CidGeneric<64> = generate_cid(chunk);
        nodes.push(DagNode {
            cid,
            data: Some(chunk.to_vec()),
            links: vec![],
        });
    }
    nodes
}

pub fn build_merkle_dag(file_data: &[u8]) -> DagNode {
    let chunk_nodes: Vec<DagNode> = chunk_file(file_data, 256 * 1024);
    let root_cid: cid::CidGeneric<64> = generate_cid(
        &chunk_nodes.iter().flat_map(|node| node.cid.to_bytes()).collect::<Vec<u8>>(),
    );
    DagNode {
        cid: root_cid,
        data: None,
        links: chunk_nodes.iter().map(|node| node.cid.clone()).collect(),
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_chunk_file() {
        let data: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let nodes: Vec<DagNode> = chunk_file(&data, 3);
        assert_eq!(nodes.len(), 4);
        assert_eq!(nodes[0].data, Some(vec![1, 2, 3]));
        assert_eq!(nodes[1].data, Some(vec![4, 5, 6]));
        assert_eq!(nodes[2].data, Some(vec![7, 8, 9]));
        assert_eq!(nodes[3].data, Some(vec![10]));
    }
    #[test]
    fn test_chunk_file_with_demo_file(){
      //  
    }
}