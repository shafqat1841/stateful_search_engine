use std::collections::HashMap;

use crate::cache_entries::CacheEntry;

#[derive(Debug)]
pub struct Cache<'file_buffer> {
    pub entries: HashMap<&'file_buffer str, CacheEntry<'file_buffer>>,
}

impl<'file_buffer> Cache<'file_buffer> {
    pub fn new() -> Self {
        let entries: HashMap<&'file_buffer str, CacheEntry<'file_buffer>> = HashMap::new();

        Cache { entries }
    }
}
