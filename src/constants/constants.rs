use cid::Cid;
use mime::{Mime, IMAGE_GIF, IMAGE_JPEG, IMAGE_PNG};

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
        is_dup: false,
    }
}

pub const _BASEURL: &str = "localhost:3035";
pub const _PORT: u16 = 8080;
pub const _ALLOWEDEXTENSIONS: [Mime; 3] = [IMAGE_PNG, IMAGE_JPEG, IMAGE_GIF];

pub const _MAX_FILE_COUNT: usize = 3;
pub const _MAX_FILE_SIZE: u64 = 10 * 1024;
pub const _LEGAL_FILE_TYPES: [Mime; 3] = [IMAGE_PNG, IMAGE_JPEG, IMAGE_GIF];
pub const _UPLOAD_DIR: &str = "uploads/";
pub const _STREAMPROTOCOLNAME: &str = "/manaslibp2p/connection/1.0.0";