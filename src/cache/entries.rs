use std::{collections::HashMap, rc::Rc};

pub use crate::{cache::cache_entries::CacheEntry, log_searcher::SearchResult};

#[derive(Debug)]
pub struct EntriesMap<'file_buffer> {
    pub entries: HashMap<Rc<String>, CacheEntry<'file_buffer>>,
}

impl<'file_buffer> EntriesMap<'file_buffer> {
    pub fn new() -> Self {
        let entries: HashMap<Rc<String>, CacheEntry<'file_buffer>> = HashMap::new();
        EntriesMap { entries }
    }

    pub fn check_query(&self, query: &Rc<String>) -> bool {
        self.entries.contains_key(query)
    }

    pub fn get_query_value(&self, query: &Rc<String>) -> Option<&Vec<SearchResult<'file_buffer>>> {
        let result: Option<&Vec<SearchResult<'file_buffer>>> = match self.entries.get(query) {
            None => None,
            Some(val) => Some(&val.values),
        };

        result
    }

    pub fn create_cache_entry(
        &mut self,
        entry_result: Vec<SearchResult<'file_buffer>>,
        node_index: usize,
    ) -> CacheEntry<'file_buffer> {
        let cache_entry: CacheEntry<'file_buffer> = CacheEntry::new(entry_result, node_index);
        cache_entry
    }

    pub fn insert_entry(
        &mut self,
        entry_result: Vec<SearchResult<'file_buffer>>,
        query: Rc<String>,
        node_index: usize,
    ) {
        let cache_entry = self.create_cache_entry(entry_result, node_index);

        let trimed_query = query;

        self.entries.insert(trimed_query, cache_entry);
    }

    pub fn get_entry_ref(&mut self, trimed_query: &Rc<String>) -> Option<&CacheEntry<'_>> {
        let entry = self.entries.get(trimed_query);
        entry
    }

    pub fn get_entries_len(&self) -> usize {
        let res = self.entries.len();
        res
    }

    pub fn remove_entry(&mut self,query: &Rc<String>) {
        self.entries.remove(query);
    }
}
