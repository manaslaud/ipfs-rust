use cid::Cid;
use multihash::Multihash;

pub fn generate_cid(data: &[u8]) -> Cid {
    const SHA2_256: u64 = 0x12;
    //create the digest from data
    let hash = Multihash::<64>::wrap(SHA2_256, data).expect("Could not hash slice");
    Cid::new_v1(0x55, hash)
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
}