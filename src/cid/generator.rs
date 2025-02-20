use cid::Cid;
use multihash::Multihash;
use std::fs::File;
use std::io::{self, Read};

const CHUNK_SIZE: usize = 1024; 

use sha2::{Sha256, Digest};

pub fn generate_cid(data: &[u8]) -> Cid {
    const SHA2_256: u64 = 0x12; 

    let hash_output = Sha256::digest(data); 

    let hash = Multihash::<64>::wrap(SHA2_256, &hash_output)
        .expect("Could not wrap SHA-256 hash in Multihash");

    Cid::new_v1(0x55, hash) // 0x55 is the raw codec
}


pub fn generate_cids_from_file(file_path: &str) -> io::Result<Vec<Cid>> {
    let mut file = File::open(file_path)?;
    let mut buffer = vec![0; CHUNK_SIZE];
    let mut cids = Vec::new();

    while let Ok(bytes_read) = file.read(&mut buffer) {
        if bytes_read == 0 {
            break; 
        }
        let chunk = &buffer[..bytes_read];
        let cid = generate_cid(chunk);
        cids.push(cid);
    }

    Ok(cids)
}

//tests for the generate_cid function
#[cfg(test)]
mod tests {
    use super::*;
    use multihash::Multihash;
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
    fn test_generate_cids_from_file() -> io::Result<()> {
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
        println!("Temp file path: {:?}", file_path);
    
        // Generate CIDs from the temporary file
        let cids = generate_cids_from_file(file_path.to_str().unwrap())?;
    
        assert!(!cids.is_empty(), "CIDs list should not be empty");
        assert!(cids.len() >= 1, "At least one CID should be generated");
    
        for cid in &cids {
            println!("{}", cid);   
        }
    
        Ok(())
    }
    
    
}
