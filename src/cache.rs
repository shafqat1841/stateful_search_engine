use std::collections::HashMap;

use crate::{cache_entries::CacheEntry, search_result::SearchResult};

#[derive(Debug)]
pub struct Cache<'file_buffer> {
    pub entries: HashMap<String, CacheEntry<'file_buffer>>,
}

impl<'file_buffer> Cache<'file_buffer> {
    pub fn new() -> Self {
        let entries: HashMap<String, CacheEntry<'file_buffer>> = HashMap::new();

        Cache { entries }
    }

    pub fn check_query(&self, query: &'file_buffer str) -> bool {
        self.entries.contains_key(query)
    }

    pub fn get_query_value(&mut self, query: &str) -> Option<&Vec<SearchResult<'file_buffer>>> {
        let result: Option<&Vec<SearchResult<'file_buffer>>> = match self.entries.get_mut(query) {
            None => None,
            Some(val) => {
                val.increase_access_count();
                Some(&val.values)
            }
        };

        result
    }

    pub fn insert_query_result(
        &mut self,
        query: String,
        entry_result: Vec<SearchResult<'file_buffer>>,
    ) {
        let mut cache_entry: CacheEntry<'file_buffer> = CacheEntry::new();
        cache_entry.insert_result(entry_result);

        self.entries.insert(query.trim().to_string(), cache_entry);

    }
}
