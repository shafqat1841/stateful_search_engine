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

    pub fn debugging_logs(&self) {
        for item in self.lru_nodes_list.iter().enumerate() {
            println!(
                "index: {:?} and key: {:?} and prev: {:?} and next: {:?}",
                item.0,
                item.1.get_key(),
                item.1.get_prev(),
                item.1.get_next()
            )
        }
        println!("free_node_slot: {:?}", self.free_node_slot);
        println!("head: {:?}", self.head);
        println!("tail: {:?}", self.tail);
    }

    pub fn get_current_index(&self) -> usize {
        if let Some(free_node_slot) = self.free_node_slot {
            free_node_slot
        } else {
            self.lru_nodes_list.len()
        }
    }

    fn get_mut_node(&mut self, index: usize) -> Option<&mut NodeSlot> {
        let node: Option<&mut NodeSlot> = self.lru_nodes_list.get_mut(index);
        node
    }

    fn update_node_next(&mut self, node_index: usize, node_next_index: Option<usize>) {
        let node_slot = self.get_mut_node(node_index);
        if let Some(node_slot) = node_slot {
            node_slot.update_next(node_next_index);
        }
    }

    fn update_node_previous(&mut self, node_index: usize, node_previousindex: Option<usize>) {
        let node_slot = self.get_mut_node(node_index);
        if let Some(node_slot) = node_slot {
            node_slot.update_previous(node_previousindex);
        }
    }

    fn insert_initial_node(&mut self, query: String, index: usize) {
        let node_slot = NodeSlot::new(query, None, None);
        self.head = Some(index);
        self.tail = Some(index);
        self.lru_nodes_list.push(node_slot);
    }

    fn is_free_node_slot_and_index_same(&self, index: usize) -> bool {
        if let Some(free_node_slot) = self.free_node_slot {
            free_node_slot == index
        } else {
            false
        }
    }

    fn fill_empty_slot(&mut self, query: String, index: usize) {
        let empty_node_slot = self.get_mut_node(index);
        if let Some(empty_node_slot) = empty_node_slot {
            empty_node_slot.make_empty_occupied(query);
        }

        self.free_node_slot = None;
    }

    fn insert_node(&mut self, query: String, index: usize) {
        if let Some(head_index) = self.head {
            self.update_node_next(head_index, Some(index));
        }
        let node_slot = NodeSlot::new(query, self.head, None);
        self.head = Some(index);
        self.lru_nodes_list.push(node_slot);
    }

    pub fn insert_new_node(&mut self, query: String, index: usize) {
        if self.lru_nodes_list.is_empty() {
            self.insert_initial_node(query, index);
        } else if self.is_free_node_slot_and_index_same(index) {
            self.fill_empty_slot(query, index)
        } else {
            self.insert_node(query, index);
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
            if let Some(prev) = prev_and_next.get_prev() {
                self.update_node_next(prev, prev_and_next.get_next())
            }
        }
    }

    fn update_current_node_next_node(&mut self, index: usize) {
        let prev_and_next: Option<PrevAndNext> = self.get_current_node_prev_and_next(index);

        if let Some(prev_and_next) = prev_and_next {
            if let Some(next) = prev_and_next.get_next() {
                self.update_node_previous(next, prev_and_next.get_prev());
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

    fn get_tail_next_index(&mut self, tail: usize) -> Option<usize> {
        let node = self.get_mut_node(tail);
        if let Some(node) = node {
            let next: Option<usize> = node.get_next();
            return next
        }
        None
    }

    fn update_current_node(&mut self, index: usize) {
        self.update_current_head_next(index);
        self.update_current_node_prev(index);
        
        self.make_current_index_head(Some(index));
        
        if let Some(tail) = self.tail {
            if index == tail {
                let new_tail_index = self.get_tail_next_index(tail);
                self.tail = new_tail_index;
            }
        }
        self.update_node_next(index, None);
    }

    fn update_nodes(&mut self, index: Option<usize>) {
        if let Some(index) = index {
            self.update_current_node_prev_node(index);
            self.update_current_node_next_node(index);
            self.update_current_node(index);
        }
    }

    fn make_index_free(&mut self, index: Option<usize>) {
        self.free_node_slot = index
    }

    fn empty_current_tail(&mut self, tail_index: usize) {
        let node = self.get_mut_node(tail_index);
        if let Some(node) = node {
            node.make_empty();
            self.make_index_free(Some(tail_index));
        }
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
            let query: Option<String> = self.get_current_tail_key();
            self.empty_current_tail(tail_index);
            return query;
        }

        None
    }
}
