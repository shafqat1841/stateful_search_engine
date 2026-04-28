use std::collections::HashMap;

use crate::{
    cache_entries::CacheEntry, constants::{CACHE_ENTRIES_LIMIT, DEVELOPMENT}, search_result::SearchResult,
};

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
            entries_limit: CACHE_ENTRIES_LIMIT,
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

    pub fn get_result(&mut self, query: &str) -> Option<&Vec<SearchResult<'file_buffer>>> {
        let result: Option<&Vec<SearchResult<'file_buffer>>> = self.get_query_value(query);

        if DEVELOPMENT {
            self.show_lru_value();
    
            self.show_access_count();
        }


        result
    }

    pub fn show_lru_value(&self) {
        match &self.lru_query {
            None => println!("lru value is: None"),
            Some(val) => {
                println!("lru value is: {:?}", val);
            }
        };
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

    fn get_cache_length(&self) -> usize {
        let res = self.entries.len();
        res
    }

    pub fn is_entries_limit_reached(&self) -> bool {
        let res = self.get_cache_length() >= self.entries_limit;
        res
    }

    pub fn get_new_lru_value(&mut self) {
        let mut new_lru: Option<(&String, &CacheEntry<'file_buffer>)> = None;
        for item in self.entries.iter() {
            let access_count = item.1.access_count;

            if let Some(val) = new_lru {
                if val.1.access_count > access_count {
                    new_lru = Some(item)
                }
            } else {
                new_lru = Some(item)
            }
        }

        if let Some(val) = new_lru {
            self.lru_query = Some(val.0.clone())
        } else {
            self.lru_query = None
        }
    }

    pub fn check_and_remove_entries(&mut self) {
        if self.is_entries_limit_reached() {
            if let Some(lru) = &self.lru_query {
                self.entries.remove(lru);
            }
            self.get_new_lru_value();
        }
    }
}
