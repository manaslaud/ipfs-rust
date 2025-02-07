use cid::Cid;
use multihash::Multihash;

const SHA2_256: u64 = 0x12;

pub fn generate_cid(data: &[u8]) -> Cid {
	let hash = Multihash::<64>::wrap(SHA2_256, data).expect("Could not hash slice");
    Cid::new_v1(0x55, hash)
}
