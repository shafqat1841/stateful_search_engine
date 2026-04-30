use std::collections::HashMap;

use crate::{
    cache_entries::CacheEntry,
    constants::{CACHE_ENTRIES_LIMIT, DEVELOPMENT},
    lru_nodes_list::LRUNodesList,
    search_result::SearchResult,
};

#[derive(Debug)]
pub struct Cache<'file_buffer> {
    pub entries: HashMap<String, CacheEntry<'file_buffer>>,
    pub lru_nodes: LRUNodesList,
    pub entries_limit: usize,
}

impl<'file_buffer> Cache<'file_buffer> {
    pub fn new() -> Self {
        let entries: HashMap<String, CacheEntry<'file_buffer>> = HashMap::new();
        let lru_nodes: LRUNodesList = LRUNodesList::new();
        Cache {
            entries,
            lru_nodes,
            entries_limit: CACHE_ENTRIES_LIMIT,
        }
    }

    pub fn check_query(&self, query: &'file_buffer str) -> bool {
        self.entries.contains_key(query)
    }

    pub fn get_query_value(&self, query: &str) -> Option<&Vec<SearchResult<'file_buffer>>> {
        let result: Option<&Vec<SearchResult<'file_buffer>>> = match self.entries.get(query) {
            None => None,
            Some(val) => Some(&val.values),
        };

        result
    }

    pub fn show_access_count(&self) {
        // println!("-------------------------------------------");
        // for (i, kv) in self.entries.iter().enumerate() {
        //     println!("key number {:?} is: {:?}", i, kv.0)
        // }
        // println!("-------------------------------------------");
        // println!("head: {:?}", self.lru_nodes.head);
        // println!("tail: {:?}", self.lru_nodes.tail);
        // println!("-------------------------------------------");
        // for item in self.lru_nodes.lru_nodes_list.iter().enumerate() {
        //     println!(
        //         "index: {:?} and key: {:?} and prev: {:?} and next: {:?}",
        //         item.0,
        //         item.1.get_key(),
        //         item.1.get_prev(),
        //         item.1.get_next()
        //     )
        // }
        // println!("-------------------------------------------");
    }

    pub fn get_result(&mut self, query: &str) -> Option<&Vec<SearchResult<'file_buffer>>> {
        let result: Option<&Vec<SearchResult<'file_buffer>>> = self.get_query_value(query);

        if DEVELOPMENT {
            self.show_access_count();
        }

        result
    }

    pub fn create_cache_entry(
        &mut self,
        entry_result: Vec<SearchResult<'file_buffer>>,
    ) -> CacheEntry<'file_buffer> {
        let node_index: usize = self.lru_nodes.get_current_index();
        let cache_entry: CacheEntry<'file_buffer> = CacheEntry::new(entry_result, node_index);
        cache_entry
    }

    pub fn insert_entry(&mut self, entry_result: Vec<SearchResult<'file_buffer>>, query: String) {
        let cache_entry = self.create_cache_entry(entry_result);

        let trimed_query = query.trim().to_string();

        self.entries.insert(trimed_query, cache_entry);
    }

    pub fn insert_new_node(&mut self, trimed_query: &str) {
        let entry = self.entries.get(trimed_query);
        match entry {
            None => {
                self.lru_nodes
                    .insert_new_node(trimed_query.to_string().clone(), None);
            }
            Some(val) => {
                self.lru_nodes
                    .insert_new_node(trimed_query.to_string().clone(), Some(val.node_index));
            }
        }
    }

    pub fn update_nodes(&mut self, query: &str) {
        let entry = self.entries.get(query);

        if let Some(val) = entry {
            self.lru_nodes.update_nodes(Some(val.node_index));
        }
    }

    fn get_cache_length(&self) -> usize {
        let res = self.entries.len();
        res
    }

    pub fn is_entries_limit_reached(&self) -> bool {
        let res = self.get_cache_length() >= self.entries_limit;
        res
    }

    pub fn remove_tail(&mut self) {
        let query = self.lru_nodes.remove_tail();
        if let Some(query) = query {
            self.entries.remove(&query);
        }
    }

    pub fn check_and_remove_entries(&mut self) {
        if self.is_entries_limit_reached() {
            self.remove_tail();
        }
    }
}
