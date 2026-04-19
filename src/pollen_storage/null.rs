use std::io;
use bimap::BiMap;
use crate::pollen_storage::{PollenIdToNameMap, PollenStorage};

pub struct NullPollenStorage;

impl PollenStorage for NullPollenStorage {
    fn get_map(&self) -> io::Result<PollenIdToNameMap> {
        Ok(BiMap::new())
    }

    fn set_map(&self, _map: &PollenIdToNameMap) -> io::Result<()> {
        Ok(())
    }
}
