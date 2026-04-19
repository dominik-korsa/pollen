use bimap::BiMap;
use crate::pollen_storage::{PollenIdToNameMap, PollenStorage};

#[allow(dead_code)]
pub struct NullPollenStorage;

impl PollenStorage for NullPollenStorage {
    fn get_map(&self) -> Result<PollenIdToNameMap, Box<dyn std::error::Error>> {
        Ok(BiMap::new())
    }

    fn set_map(&self, _map: &PollenIdToNameMap) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
