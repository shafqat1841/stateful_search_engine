use std::collections::HashMap;

use crate::{cache_entries::CacheEntry, search_result::SearchResult};

#[derive(Debug)]
pub struct Cache<'file_buffer> {
    pub lru_query: Option<String>,
    pub entries: HashMap<String, CacheEntry<'file_buffer>>,
    pub entries_limit: usize,
}

impl<'file_buffer> Cache<'file_buffer> {
    pub fn new() -> Self {
        let entries: HashMap<String, CacheEntry<'file_buffer>> = HashMap::new();

        Cache {
            entries,
            lru_query: None,
            entries_limit: 5,
        }
    }

    pub fn check_query(&self, query: &'file_buffer str) -> bool {
        self.entries.contains_key(query)
    }

    pub fn update_query_access_count_value(&mut self, query: &str) {
        if let Some(val) = self.entries.get_mut(query) {
            val.increase_access_count();
        }
    }

    pub fn get_query_value(&self, query: &str) -> Option<&Vec<SearchResult<'file_buffer>>> {
        let result: Option<&Vec<SearchResult<'file_buffer>>> = match self.entries.get(query) {
            None => None,
            Some(val) => Some(&val.values),
        };

        result
    }

    pub fn show_access_count(&self) {
        for (i, kv) in self.entries.iter().enumerate() {
            println!(
                "key number {:?} is: {:?} and its access count is: {:?}",
                i, kv.0, kv.1.access_count
            )
        }
    }

    pub fn get_result(
        &mut self,
        query: &str,
    ) -> (String, Option<&Vec<SearchResult<'file_buffer>>>) {
        self.update_query_access_count_value(query);

        let lru = self.get_lru();

        let result: Option<&Vec<SearchResult<'file_buffer>>> = self.get_query_value(query);

        self.show_access_count();

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
        self.check_and_remove_entries();
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

    fn get_cache_length(&self) -> usize {
        let res =self.entries.len();
        println!("file: cache.rs ~ line 126 ~ fnget_cache_length ~ res : {} ", res);
        res
    }

    pub fn is_entries_limit_reached(&self) -> bool {
        let res =self.get_cache_length() >= self.entries_limit;
        println!("file: cache.rs ~ line 132 ~ pubfnis_entries_limit_reached ~ res : {} ", res);
        res
    }

    pub fn check_and_remove_entries(&mut self) {
        if self.is_entries_limit_reached() {
            if let Some(lru) = &self.lru_query {
                println!("file: cache.rs ~ line 139 ~ ifletSome ~ lru : {} ", lru);
                self.entries.remove(lru);
            }
        }
    }
}
