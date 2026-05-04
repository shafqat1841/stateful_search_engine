use std::rc::Rc;

use crate::lru_nodes_list::prev_and_next::PrevAndNext;

#[derive(Debug)]
pub struct LRUNode {
    key: Rc<String>,
    prev_next: PrevAndNext,
}

impl LRUNode {
    pub fn new(key: Rc<String>, prev: Option<usize>, next: Option<usize>) -> LRUNode {
        // LRUNode { key, prev, next }
        let prev_next = PrevAndNext::new(prev, next);
        LRUNode { key, prev_next }
    }

    pub fn get_key(&self) -> Rc<String> {
        // self.key.clone()
        self.key.clone()
    }

    pub fn get_prev(&self) -> Option<usize> {
        let prev = self.prev_next.get_prev();
        prev
    }

    pub fn get_next(&self) -> Option<usize> {
        let next = self.prev_next.get_next();
        next
    }

    pub fn update_previous(&mut self, index: Option<usize>) {
        self.prev_next.update_prev(index);
    }
    
    pub fn update_next(&mut self, index: Option<usize>) {
        self.prev_next.update_next(index);
    }

    pub fn get_next_and_prev(&self) -> PrevAndNext {
        self.prev_next
    }
}
