use cid::Cid;

use crate::{cid::generate_cid, storage::MerkleNode};

const _DEFAULTDATA: [u8; 5] = [1, 2, 3, 4, 5];

pub fn default_cid() -> Cid {
    generate_cid(&_DEFAULTDATA)
}

pub fn default_merkle_node() -> MerkleNode {
    MerkleNode {
        cid: default_cid(),
        data: Some(_DEFAULTDATA.to_vec()),
        links: Vec::new(),
    }
}
