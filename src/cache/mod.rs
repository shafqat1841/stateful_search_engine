pub mod cache_entries;
mod entries;

pub use crate::{
    cache::entries::EntriesMap,
    constants::{CACHE_ENTRIES_LIMIT, DEVELOPMENT},
    log_searcher::SearchResult,
    lru_nodes_list::LRUNodesList,
};

#[derive(Debug)]
pub struct Cache<'file_buffer> {
    pub entries: EntriesMap<'file_buffer>,
    pub lru_nodes: LRUNodesList,
    pub entries_limit: usize,
}

impl<'file_buffer> Cache<'file_buffer> {
    pub fn new() -> Self {
        let entries = EntriesMap::new();
        let lru_nodes: LRUNodesList = LRUNodesList::new();
        Cache {
            entries,
            lru_nodes,
            entries_limit: CACHE_ENTRIES_LIMIT,
        }
    }

    pub fn check_query(&self, query: &'file_buffer str) -> bool {
        self.entries.check_query(query)
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
        let result: Option<&Vec<SearchResult<'file_buffer>>> = self.entries.get_query_value(query);

        if DEVELOPMENT {
            self.show_access_count();
        }

        result
    }

    pub fn insert_entry(&mut self, entry_result: Vec<SearchResult<'file_buffer>>, query: String) {
        let node_index: usize = self.lru_nodes.get_current_index();

        self.entries.insert_entry(entry_result, query, node_index)
    }

    pub fn insert_new_node(&mut self, trimed_query: &str) {
        let entry = self.entries.get_entry_ref(trimed_query);
        self.lru_nodes.insert_entry(trimed_query, entry);
    }

    pub fn update_nodes(&mut self, query: &str) {
        let entry = self.entries.get_entry_ref(query);

        self.lru_nodes.update_nodes_by_entry(entry);
    }

    fn get_cache_length(&self) -> usize {
        let res = self.entries.get_entries_len();
        res
    }

    pub fn is_entries_limit_reached(&self) -> bool {
        let res = self.get_cache_length() >= self.entries_limit;
        res
    }

    pub fn remove_tail(&mut self) {
        let query = self.lru_nodes.remove_tail();
        if let Some(query) = query {
            self.entries.remove_entry(&query);
        }
    }

    pub fn check_and_remove_entries(&mut self) {
        if self.is_entries_limit_reached() {
            self.remove_tail();
        }
    }
}
