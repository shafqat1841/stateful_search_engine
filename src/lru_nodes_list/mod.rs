mod lru_node;

use crate::{cache::cache_entries::CacheEntry, lru_nodes_list::lru_node::{LRUNode, PrevAndNext}};

#[derive(Debug)]
pub struct LRUNodesList {
    pub lru_nodes_list: Vec<LRUNode>,
    pub head: Option<usize>,
    pub tail: Option<usize>,
}

impl LRUNodesList {
    pub fn new() -> LRUNodesList {
        let lru_nodes_list: Vec<LRUNode> = Vec::new();
        let head = None;
        let tail = Some(0);

        LRUNodesList {
            lru_nodes_list,
            head,
            tail,
        }
    }

    pub fn get_current_index(&self) -> usize {
        self.lru_nodes_list.len()
    }

    fn insert_initial_node(&mut self, query: String, index: Option<usize>) {
        let lru_node = LRUNode::new(query, None, None);
        self.head = index;
        self.tail = index;
        self.lru_nodes_list.push(lru_node);
    }

    fn insert_node(&mut self, query: String, index: Option<usize>) {
        if let Some(head) = self.head {
            let head_node = self.lru_nodes_list.get_mut(head);
            if let Some(head_node) = head_node {
                head_node.update_next(index);
            }
        }

        let lru_node = LRUNode::new(query, self.head, None);
        self.head = index;
        self.lru_nodes_list.push(lru_node);
    }

    pub fn insert_new_node(&mut self, query: String, index: Option<usize>) {
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

    pub fn get_current_node_prev_and_next(
        &mut self,
        index: usize,
    ) -> Option<PrevAndNext> {
        let current_node: Option<&mut LRUNode> = self.lru_nodes_list.get_mut(index);

        if let Some(current_node) = current_node {
            let next_and_prev: PrevAndNext = current_node.get_next_and_prev();

            return Some(next_and_prev);
        }

        None
    }

    pub fn update_prev_node(&mut self, prev: usize, next_node_index: Option<usize>) {
        let prev_node_opt = self.lru_nodes_list.get_mut(prev);
        if let Some(prev_node_val) = prev_node_opt {
            prev_node_val.update_next(next_node_index);
        }
    }

    pub fn update_next_node(&mut self, next: usize, prev_node_index: Option<usize>) {
        let next_node_opt = self.lru_nodes_list.get_mut(next);
        if let Some(next_node_val) = next_node_opt {
            next_node_val.update_previous(prev_node_index);
        }
    }

    pub fn update_current_node_prev_node(&mut self, index: usize) {
        let prev_and_next: Option<PrevAndNext> = self.get_current_node_prev_and_next(index);
        if let Some(prev_and_next) = prev_and_next {
            if let Some(prev) = prev_and_next.prev {
                self.update_prev_node(prev, prev_and_next.next)
            }
        }
    }

    pub fn update_current_node_next_node(&mut self, index: usize) {
        let prev_and_next: Option<PrevAndNext> = self.get_current_node_prev_and_next(index);

        if let Some(prev_and_next) = prev_and_next {
            if let Some(next) = prev_and_next.next {
                self.update_next_node(next, prev_and_next.prev)
            }
        }
    }

    pub fn update_current_node_next(&mut self, next: Option<usize>, current_node_index: usize) {
        let current_node_opt = self.lru_nodes_list.get_mut(current_node_index);
        if let Some(current_node_val) = current_node_opt {
            current_node_val.update_next(next);
        }
    }

    pub fn update_current_head_next(&mut self, index: usize) {
        if let Some(head) = self.head {
            let current_head = self.lru_nodes_list.get_mut(head);
            if let Some(current_head) = current_head {
                current_head.update_next(Some(index));
            }
        }
    }

    pub fn update_current_node_prev(&mut self, index: usize) {
        let current_node = self.lru_nodes_list.get_mut(index);
        if let Some(current_node) = current_node {
            if let Some(head) = self.head {
                current_node.update_previous(Some(head));
                self.head = Some(index);
            }
        }
    }

    pub fn update_current_node(&mut self, index: usize) {
        self.update_current_node_next(None, index);
        self.update_current_head_next(index);
        self.update_current_node_prev(index);
    }

    pub fn update_nodes(&mut self, index: Option<usize>) {
        if let Some(index) = index {
            self.update_current_node_prev_node(index);
            self.update_current_node_next_node(index);
            self.update_current_node(index);
        }
    }

    pub fn get_tail_node_next_index(&mut self, tail_index: usize) -> Option<usize> {
        let current_tail = self.lru_nodes_list.get_mut(tail_index);
        if let Some(current_tail) = current_tail {
            current_tail.get_next()
        } else {
            None
        }
    }

    pub fn make_node_tail(&mut self, node_index: Option<usize>) {
        if let Some(node_index) = node_index {
            let current_tail = self.lru_nodes_list.get_mut(node_index);

            if let Some(current_tail) = current_tail {
                current_tail.update_previous(None);
            }
        }
    }

    pub fn remove_current_tail(&mut self, tail_index: usize) {
        self.lru_nodes_list.remove(tail_index);
    }

    pub fn make_node_index_tail(&mut self, node_index: Option<usize>) {
        self.tail = node_index;
    }

    pub fn get_current_tail_key(&mut self) -> Option<String> {
        if let Some(tail) = self.tail {
            let tail_node = self.lru_nodes_list.get(tail);
            if let Some(tail_node) = tail_node {
                let res = tail_node.get_key();
                Some(res)
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
            // self.remove_current_tail(tail_index);
            self.make_node_index_tail(next_node_index);
            return query;
        }

        None
    }
}
