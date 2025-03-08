use cid::Cid;
use multihash::Multihash;
use std::fs::File;
use std::io::{self, Read};
use crate::storage::MerkleNode;

const CHUNK_SIZE: usize = 1024; 

use sha2::{Sha256, Digest};

pub fn generate_cid(data: &[u8]) -> Cid {
    const SHA2_256: u64 = 0x12; 

    let hash_output = Sha256::digest(data); 

    let hash = Multihash::<64>::wrap(SHA2_256, &hash_output)
        .expect("Could not wrap SHA-256 hash in Multihash");

    Cid::new_v1(0x55, hash) // 0x55 is the raw codec
}


pub fn generate_leaves_from_file(file_path: &str) -> io::Result<Vec<MerkleNode>> {
    let mut file = File::open(file_path)?;
    let mut buffer = vec![0; CHUNK_SIZE];
    let mut leaves = Vec::new();
    while let Ok(bytes_read) = file.read(&mut buffer) {
        if bytes_read == 0 {
            break; 
        }
        let chunk = &buffer[..bytes_read];
        let cid = generate_cid(chunk);
        let node:MerkleNode=MerkleNode{
            cid,
            data:Some(chunk.to_vec()),
            links:vec![],
            is_dup:false
        };
        leaves.push(node);
    }

    Ok(leaves)
}

//tests for the generate_cid function
#[cfg(test)]
mod tests {
    use super::*;
    use multihash::Multihash;
    use crate::storage::dag::generate_merkle_tree;
    use std::fs::File;
    use std::io::{self, Read};
    #[test]
    fn test_generate_cid() {
        const SHA2_256: u64 = 0x12;
        let data: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        let hash: Multihash<64> =
            Multihash::<64>::wrap(SHA2_256, &data).expect("Could not hash slice");
        let cid = Cid::new_v1(0x55, hash);
        assert_eq!(generate_cid(&data), cid);
    }
    #[test]
    fn test_generate_leaves_from_file() -> io::Result<()> {
        use tempfile::NamedTempFile;
        use std::io::{Write, BufWriter};
    
        // Create a temporary file with some sample data
        let temp_file = NamedTempFile::new()?; 
        let mut writer = BufWriter::new(&temp_file);  // Buffered writer for efficiency
    
        for x in 0..100 {
            writeln!(writer, "Chunk {}: Some random data", x)?;
        }
        writer.flush()?;  
    
        // Get the temp file's path and convert it to a string
        let file_path = temp_file.path();
    
        // Generate leaves from the temporary file
        let leaves = generate_leaves_from_file(file_path.to_str().unwrap())?;
    
        assert!(!leaves.is_empty(), "leaves list should not be empty");
        assert!(leaves.len() >= 1, "At least one leaf should be generated");
    
        let tree=generate_merkle_tree(leaves.clone(), "png").unwrap();
        for node in tree {
            print!("{:?}",node.data)
        }
        Ok(())
    }
    #[test]
    fn test_generate_retrieve_file() {
        let test_file_path = "readme.md"; 

        let mut file = File::open(test_file_path).expect("Failed to open manas.txt");

        let mut content = String::new();
        file.read_to_string(&mut content).expect("Failed to read file");

        let nodes = generate_leaves_from_file(test_file_path).expect("Failed to generate nodes");

        // Verify that nodes are generated
        assert!(!nodes.is_empty(), "No leaves were generated from the file");

        // Ensure each node has a valid CID
        for node in &nodes {
            assert!(!node.cid.to_string().is_empty(), "Generated node has an empty CID");
            assert!(node.data.is_some(), "Generated node has no data");
        }

        let tree=generate_merkle_tree(nodes, "md").unwrap();
        for x in tree {
            println!("{:?}",x.links);
        }

    
    }
    
}
