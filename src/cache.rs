use std::collections::HashMap;

use crate::{cache_entries::CacheEntry, search_result::SearchResult};

#[derive(Debug)]
pub struct Cache<'file_buffer> {
    pub lru_query: Option<String>,
    pub entries: HashMap<String, CacheEntry<'file_buffer>>,
}

impl<'file_buffer> Cache<'file_buffer> {
    pub fn new() -> Self {
        let entries: HashMap<String, CacheEntry<'file_buffer>> = HashMap::new();

        Cache {
            entries,
            lru_query: None,
        }
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

    pub fn get_result(
        &mut self,
        query: &str,
    ) -> (String, Option<&Vec<SearchResult<'file_buffer>>>) {
        let lru = self.get_lru();

        let result: Option<&Vec<SearchResult<'file_buffer>>> = self.get_query_value(query);

        (lru, result)
    }

    pub fn get_lru(&self) -> String {
        let val = match &self.lru_query {
            None => "None".to_string(),
            Some(val) => {
                let res = val.clone();
                res
            }
        };

        val
    }

    pub fn create_default_cache_entry(
        &self,
        entry_result: Vec<SearchResult<'file_buffer>>,
    ) -> CacheEntry<'file_buffer> {
        let mut cache_entry: CacheEntry<'file_buffer> = CacheEntry::new();
        cache_entry.insert_result(entry_result);
        cache_entry
    }

    pub fn insert_entry(&mut self, entry_result: Vec<SearchResult<'file_buffer>>, query: String) {
        let cache_entry = self.create_default_cache_entry(entry_result);

        let trimed_query = query.trim().to_string();

        self.entries.insert(trimed_query, cache_entry);
    }

    pub fn add_new_query_in_lrc_and_entries(
        &mut self,
        trimed_query: String,
        entry_result: Vec<SearchResult<'file_buffer>>,
    ) {
        let query_clone = trimed_query.clone();
        self.insert_entry(entry_result, query_clone);

        match &self.lru_query {
            None => {
                self.lru_query = Some(trimed_query);
            }
            Some(val) => {
                let lru_entry_opt = self.entries.get(val);
                let new_entry_opt = self.entries.get(&trimed_query);

                if let Some(new_entry) = new_entry_opt {
                    if let Some(lru_entry) = lru_entry_opt {
                        if new_entry.access_count < lru_entry.access_count {
                            self.lru_query = Some(trimed_query);
                        }
                    }
                }
            }
        };
    }
}
