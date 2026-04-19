use std::path::PathBuf;
use crate::pollen_storage::{PollenIdToNameMap, PollenStorage};

pub struct FilePollenStorage {
    path: PathBuf,
}

impl FilePollenStorage {
    pub fn new(path: PathBuf) -> Self {
        FilePollenStorage { path }
    }
}

impl PollenStorage for FilePollenStorage {
    fn get_map(&self) -> Result<PollenIdToNameMap, Box<dyn std::error::Error>> {
        let map = match std::fs::File::open(&self.path) {
            Ok(file) => {
                serde_json::from_reader(file)?
            }
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                PollenIdToNameMap::new()
            }
            Err(err) => {
                return Err(err.into());
            }
        };
        Ok(map)
    }

    fn set_map(&self, map: &PollenIdToNameMap) -> Result<(), Box<dyn std::error::Error>> {
        let file = std::fs::File::create(&self.path)?;
        serde_json::to_writer(file, map)?;
        Ok(())
    }
}