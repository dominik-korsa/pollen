pub mod null;

use std::io;
use bimap::BiMap;
use slugify::slugify;

pub type PollenIdToNameMap = BiMap<String, String>;

pub trait PollenStorage {
    /// Get the map of pollen IDs to their names
    fn get_map(&self) -> io::Result<PollenIdToNameMap>;

    /// Set the map of pollen IDs to their names
    fn set_map(&self, map: &PollenIdToNameMap) -> io::Result<()>;
}

pub fn generate_id_for_name(name: &str) -> String {
    slugify!(name, separator = "_")
}