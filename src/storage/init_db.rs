use fjall::{Config, Error, Keyspace, PartitionCreateOptions, PartitionHandle};
use std::fs;
use std::path::PathBuf;

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
}
