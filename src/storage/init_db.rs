use fjall::{Config, Error, Keyspace, PartitionCreateOptions, PartitionHandle};
use std::fs;
use std::path::PathBuf;
use serde_json;
use crate::storage::MerkleNode;

pub async fn init_db(path: String) -> Result<PartitionHandle, Error> {
    let folder: PathBuf = PathBuf::from(path);
    if !folder.exists() {
        fs::create_dir_all(&folder)?;
    }
    let keyspace: Keyspace = Config::new(folder).open()?;
    let items: PartitionHandle =
        keyspace.open_partition("slices", PartitionCreateOptions::default())?;
    Ok(items)
}

/*
tldr; how it works
take a tree, each node is added iteratively to the kv store, with key being its cid value
allows for deduplication since if 2<= files share a common dag node, only 1 entry is recorded.
if two or more nodes have the same CID, regardless of whether they're leaf or inner nodes,
it means that the child node(s)/data chunks are also the same, since a node is hashed based on 
it's child nodes & collision is technically infeasible
Proof: Identical CIDs ⟹ Identical Content
Claim: If two nodes N1N1​ and N2N2​ have the same CID, then their data and children must be identical.
We assume that N1≠N2​, meaning they are different in some way (either in data or child nodes).
Proof 
    Suppose we have two different nodes N1​ and N2​ with the same CID:
    H(data1∥C1(1)∥C2(1)∥...∥Cn(1))=H(data2∥C1(2)∥C2(2)∥...∥Cm(2))
    H(data1​∥C1(1)​∥C2(1)​∥...∥Cn(1)​)=H(data2​∥C1(2)​∥C2(2)​∥...∥Cm(2)​)
    Since H is a cryptographic hash function, it is collision-resistant:
        This means it is computationally infeasible to find two different inputs that hash to the same output.
    For CID(N1)=CID(N2)CID(N1​)=CID(N2​) to hold, the inputs must be identical:
        data1=data2
        Ci(1)=Ci(2) for all i
        n=m (same number of children)
    Contradiction: We assumed N1≠N2​, but we just proved that their content must be identical.
    Therefore, if two nodes have the same CID, they must be identical. 
 */
pub async fn store_file(tree:Vec<MerkleNode>) {
    let items = init_db(String::from("./tmp/data")).await.unwrap();
    for x in tree.iter() {
        let value=serde_json::to_string(x).unwrap(); 
        let output = items.insert(x.cid.to_string(), value);
        match output {
            Ok(()) => print!("Success"),
            Err(error) => print!("Error, {:?}",error)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::init_db;

    #[tokio::test]
    async fn test_insert() {
        let items = init_db(String::from("./tmp/data")).await.unwrap();
        let _ = items.insert("key", "value").unwrap();
        let ret: fjall::Slice = items.get("key").unwrap().unwrap();
        let value_from_db = String::from_utf8_lossy(ret.as_ref()).to_string();
        assert_eq!(value_from_db, "value");
    }
    #[tokio::test]
    async fn test_file_insert() {

    }
}
