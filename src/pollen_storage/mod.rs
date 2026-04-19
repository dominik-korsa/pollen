mod null;
mod file;

use bimap::BiMap;
use slugify::slugify;

pub type PollenIdToNameMap = BiMap<String, String>;

pub trait PollenStorage {
    /// Get the map of pollen IDs to their names
    fn get_map(&self) -> Result<PollenIdToNameMap, Box<dyn std::error::Error>>;

    /// Set the map of pollen IDs to their names
    fn set_map(&self, map: &PollenIdToNameMap) -> Result<(), Box<dyn std::error::Error>>;
}

pub fn generate_id_for_name(name: &str) -> String {
    slugify!(name, separator = "_")
}

#[allow(unused_imports)]
pub use null::NullPollenStorage;
pub use file::FilePollenStorage;