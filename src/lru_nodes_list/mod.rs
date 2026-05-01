mod lru_node;
mod node_slot;
mod prev_and_next;

use crate::{
    cache::cache_entries::CacheEntry,
    lru_nodes_list::{node_slot::NodeSlot, prev_and_next::PrevAndNext},
};

#[derive(Debug)]
pub struct LRUNodesList {
    lru_nodes_list: Vec<NodeSlot>,
    free_node_slot: Option<usize>,
    head: Option<usize>,
    tail: Option<usize>,
}

impl LRUNodesList {
    pub fn new() -> LRUNodesList {
        let lru_nodes_list: Vec<NodeSlot> = Vec::new();
        let free_node_slot = None;
        let head = None;
        let tail = Some(0);

        LRUNodesList {
            lru_nodes_list,
            free_node_slot,
            head,
            tail,
        }
    }

    pub fn get_current_index(&self) -> usize {
        self.lru_nodes_list.len()
    }

    fn get_mut_node(&mut self, index: usize) -> Option<&mut NodeSlot> {
        let node: Option<&mut NodeSlot> = self.lru_nodes_list.get_mut(index);
        node
    }

    fn update_node_next(&mut self, node_index: usize, update_index: Option<usize>) {
        let node_slot = self.get_mut_node(node_index);
        if let Some(node_slot) = node_slot {
            node_slot.update_next(update_index);
        }
    }

    fn update_node_previous(&mut self, index: usize, update_index: Option<usize>) {
        let node_slot = self.get_mut_node(index);
        if let Some(node_slot) = node_slot {
            node_slot.update_previous(update_index);
        }
    }

    fn insert_initial_node(&mut self, query: String, index: Option<usize>) {
        let node_slot = NodeSlot::new(query, None, None);
        self.head = index;
        self.tail = index;
        self.lru_nodes_list.push(node_slot);
    }

    fn insert_node(&mut self, query: String, index: Option<usize>) {
        if let Some(head_index) = self.head {
            self.update_node_next(head_index, index);
        }

        let node_slot = NodeSlot::new(query, self.head, None);
        self.head = index;
        self.lru_nodes_list.push(node_slot);
    }

    fn insert_new_node(&mut self, query: String, index: Option<usize>) {
        if self.lru_nodes_list.is_empty() {
            self.insert_initial_node(query, index);
        } else {
            self.insert_node(query, index);
        }
    }

    pub fn insert_entry(&mut self, trimed_query: &str, entry: Option<&CacheEntry<'_>>) {
        match entry {
            None => {
                self.insert_new_node(trimed_query.to_string().clone(), None);
            }
            Some(val) => {
                self.insert_new_node(trimed_query.to_string().clone(), Some(val.node_index));
            }
        }
    }

    pub fn update_nodes_by_entry(&mut self, entry: Option<&CacheEntry<'_>>) {
        if let Some(val) = entry {
            self.update_nodes(Some(val.node_index));
        }
    }

    fn get_current_node_prev_and_next(&mut self, index: usize) -> Option<PrevAndNext> {
        let node = self.get_mut_node(index);

        if let Some(node) = node {
            let next_and_prev = node.get_next_and_prev();

            return next_and_prev;
        }

        None
    }

    fn update_current_node_prev_node(&mut self, index: usize) {
        let prev_and_next: Option<PrevAndNext> = self.get_current_node_prev_and_next(index);
        if let Some(prev_and_next) = prev_and_next {
            if let Some(prev) = prev_and_next.prev {
                self.update_node_next(prev, prev_and_next.next)
            }
        }
    }

    fn update_current_node_next_node(&mut self, index: usize) {
        let prev_and_next: Option<PrevAndNext> = self.get_current_node_prev_and_next(index);

        if let Some(prev_and_next) = prev_and_next {
            if let Some(next) = prev_and_next.next {
                self.update_node_previous(next, prev_and_next.prev)
            }
        }
    }

    fn update_current_head_next(&mut self, index: usize) {
        if let Some(head) = self.head {
            self.update_node_next(head, Some(index));
        }
    }

    fn update_current_node_prev(&mut self, index: usize) {
        if let Some(head) = self.head {
            let node_slot = self.get_mut_node(index);
            if let Some(node_slot) = node_slot {
                node_slot.update_previous(Some(head));
            }
        }
    }

    fn make_current_index_head(&mut self, index: Option<usize>) {
        self.head = index;
    }

    fn update_current_node(&mut self, index: usize) {
        self.update_node_next(index, None);
        self.update_current_head_next(index);
        self.update_current_node_prev(index);
        self.make_current_index_head(Some(index))
    }

    fn update_nodes(&mut self, index: Option<usize>) {
        if let Some(index) = index {
            self.update_current_node_prev_node(index);
            self.update_current_node_next_node(index);
            self.update_current_node(index);
        }
    }

    fn get_tail_node_next_index(&mut self, tail_index: usize) -> Option<usize> {
        let node_slot = self.get_mut_node(tail_index);
        if let Some(node_slot) = node_slot {
            node_slot.get_next()
        } else {
            None
        }
    }

    fn make_node_tail(&mut self, node_index: Option<usize>) {
        if let Some(node_index) = node_index {
            self.update_node_previous(node_index, None);
        }
    }

    fn make_index_free(&mut self, index: Option<usize>) {
        self.free_node_slot = index
    }

    fn remove_current_tail(&mut self, tail_index: usize) {
        let node = self.get_mut_node(tail_index);
        if let Some(node) = node {
            node.make_empty();
            self.make_index_free(Some(tail_index));
        }
    }

    fn make_node_index_tail(&mut self, node_index: Option<usize>) {
        self.tail = node_index;
    }

    fn get_current_tail_key(&mut self) -> Option<String> {
        if let Some(tail) = self.tail {
            let tail_node = self.lru_nodes_list.get(tail);
            if let Some(tail_node) = tail_node {
                let res = tail_node.get_key();
                res
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn remove_tail(&mut self) -> Option<String> {
        if let Some(tail_index) = self.tail {
            let next_node_index = self.get_tail_node_next_index(tail_index);
            self.make_node_tail(next_node_index);
            let query: Option<String> = self.get_current_tail_key();
            self.remove_current_tail(tail_index);
            self.make_node_index_tail(next_node_index);
            return query;
        }

        None
    }
}
