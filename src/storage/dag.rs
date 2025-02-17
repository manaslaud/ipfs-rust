use cid::Cid;
//for custom error definations, formatting and handling
use crate::cid::generator::generate_cid;
use thiserror::Error;
const LEFT: &str = "left";
const RIGHT: &str = "right";
const NOT_FOUND: &str = "not found";

//implement custom error functions
#[derive(Debug, Error)]
pub enum DagErrors {
    #[error("I/O error occurred: {0}")]
    IoError(#[from] std::io::Error), // Automatically implements `From<std::io::Error>`

    #[error("Failed to parse integer: {0}")]
    ParseError(#[from] std::num::ParseIntError), // Automatically implements `From<std::num::ParseIntError>`

    #[error("The vector of leaves was empty")]
    EmptyError, // A variant without additional data
}
//checks whether vector of cid's is even, if not duplicate the final cid
fn ensure_even(data: &mut Vec<Cid>) -> bool {
    if data.len() == 0 {
        return false;
    }
    if data.len() % 2 == 0 {
        return true;
    }
    let last = data.last().unwrap();
    data.push(last.clone());
    return false;
}

//get the direction of leaf node in the merkle tree
fn get_leaf_node_direction(hash: Cid, merkle_tree: &Vec<Vec<Cid>>) -> &str {
    let leaf_hashes = &merkle_tree[0]; // gets the first row of the merkle tree (leaves)
    if leaf_hashes.len() == 0 {
        return NOT_FOUND;
    }
    for idx in 0..leaf_hashes.len() {
        if leaf_hashes[idx] == hash {
            if idx % 2 == 0 {
                return LEFT;
            } else {
                return RIGHT;
            }
        }
    }
    //handle the case where the hash is not found
    NOT_FOUND
}

//generating the merkle root from the base leaf hashes vector
fn generate_merkle_root(leaves: &mut Vec<Cid>) -> Result<Cid, DagErrors> {
    if leaves.is_empty() {
        return Err(DagErrors::EmptyError);
    }

    ensure_even(leaves); // Ensure even number of leaves

    let mut combined_hashes: Vec<Cid> = Vec::new();

    for n in (0..leaves.len()).step_by(2) {
        let hash1 = leaves[n].hash().digest();
        let hash2 = leaves[n + 1].hash().digest();

        let mut combined_hash_data = Vec::new();
        combined_hash_data.extend_from_slice(hash1);
        combined_hash_data.extend_from_slice(hash2);

        let combined_hash = generate_cid(&combined_hash_data);
        combined_hashes.push(combined_hash);
    }

    // If we reduced the tree to a single root, return it
    if combined_hashes.len() == 1 {
        return Ok(combined_hashes[0]);
    }

    generate_merkle_root(&mut combined_hashes)
}

//generating the merkle tree entirely
// level 0 is the leaves level and so on
pub fn generate_merkle_tree(leaves: Vec<Cid>) -> Result<Vec<Vec<Cid>>, DagErrors> {
    if leaves.is_empty() {
        return Err(DagErrors::EmptyError);
    }

    let mut tree: Vec<Vec<Cid>> = Vec::new();
    tree.push(leaves.clone()); // Store initial leaves layer

    generate(leaves, &mut tree);

    Ok(tree)
}

fn generate(mut hashes: Vec<Cid>, tree: &mut Vec<Vec<Cid>>) {
    if hashes.len() == 1 {
        return; // Base case: stop when only one hash remains
    }

    ensure_even(&mut hashes); // Ensure an even number of leaves

    let mut combined_hashes: Vec<Cid> = Vec::new();
    for n in (0..hashes.len()).step_by(2) {
        let hash1 = hashes[n].hash().digest();
        let hash2 = hashes[n + 1].hash().digest();

        let mut combined_hash_data = Vec::new();
        combined_hash_data.extend_from_slice(hash1);
        combined_hash_data.extend_from_slice(hash2);

        let combined_hash = generate_cid(&combined_hash_data);
        combined_hashes.push(combined_hash);
    }

    tree.push(combined_hashes.clone()); // Store the new level in the tree
    generate(combined_hashes, tree); // Recursively compute the next level
}

#[cfg(test)]
mod tests {
    use super::*;
    use multihash::Multihash;
    #[test]
    fn test_ensure_even() {
        const SHA2_256: u64 = 0x12;
        let data: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        let hash: Multihash<64> =
            Multihash::<64>::wrap(SHA2_256, &data).expect("Could not hash slice");
        let mut cids: Vec<Cid> = Vec::new();
        cids.push(Cid::new_v1(0x55, hash));
        // cids.push(Cid::new_v1(0x55, hash));
        assert_eq!(ensure_even(&mut cids), false)
        // assert_eq!(ensure_even(&mut cids),true)
    }
    #[test]
    fn test_generate_merkle_root() {
        let data1: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        let cid1 = generate_cid(&data1);
        let data2: [u8; 8] = [0x66, 0x50, 0x4E, 0x47, 0x02, 0x0A, 0x1A, 0x1A];
        let cid2 = generate_cid(&data2);
        let data3: [u8; 8] = [0x81, 0x50, 0x4E, 0x47, 0x0B, 0x0A, 0x1A, 0x0A];
        let cid3 = generate_cid(&data3);
        let data4: [u8; 8] = [0x84, 0x40, 0x4E, 0x47, 0x0A, 0x0A, 0x1A, 0x9A];
        let cid4 = generate_cid(&data4);
        let mut cids: Vec<Cid> = Vec::new();
        cids.push(cid1);
        cids.push(cid2);
        cids.push(cid3);
        cids.push(cid4);
        let root = generate_merkle_root(&mut cids).unwrap();
        println!("{:?}", root);
    }
    #[test]
    fn test_generate_merkle_tree() {
        let data1: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        let cid1 = generate_cid(&data1);
        let data2: [u8; 8] = [0x66, 0x50, 0x4E, 0x47, 0x02, 0x0A, 0x1A, 0x1A];
        let cid2 = generate_cid(&data2);
        let data3: [u8; 8] = [0x81, 0x50, 0x4E, 0x47, 0x0B, 0x0A, 0x1A, 0x0A];
        let cid3 = generate_cid(&data3);
        let data4: [u8; 8] = [0x84, 0x40, 0x4E, 0x47, 0x0A, 0x0A, 0x1A, 0x9A];
        let cid4 = generate_cid(&data4);
        let mut cids: Vec<Cid> = Vec::new();
        cids.push(cid1);
        cids.push(cid2);
        cids.push(cid3);
        cids.push(cid4);
        let tree = generate_merkle_tree(cids.clone()).unwrap();
        println!("{:?}", tree);
        //compare root obtained here with root obtained in the previous test
        let root = generate_merkle_root(&mut cids.clone()).unwrap();
        assert_eq!(tree.last().unwrap().first().unwrap(), &root);
    }
}
