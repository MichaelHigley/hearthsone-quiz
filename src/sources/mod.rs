use anyhow::Result;
use serde::Serialize;

/// Implements and defines attributes for searchable loading a set of items into a searchable database
pub trait ItemSource<const ATTRIBUTE_COUNT: usize> {
    /// Search item type to serialize before dumping into a searchable database
    type Item: Serialize;

    // we store this so we can access it from loading macros
    const ATTRIBUTE_COUNT: usize = ATTRIBUTE_COUNT;
    const STORAGE_INDEX: &'static str;
    const PRIMARY_KEY: &'static str;

    /// array of attribute keys to take from the serialized data type
    const SEARCHABLE_ATTRIBUTES: [&'static str; ATTRIBUTE_COUNT];

    /// fetch the list of searchable items to persist into the database
    fn fetch_items() -> Result<Vec<Self::Item>>;
}

mod hearthstone;
pub use hearthstone::Hearthstone;
