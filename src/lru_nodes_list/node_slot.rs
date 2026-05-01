use crate::lru_nodes_list::{lru_node::LRUNode, prev_and_next::PrevAndNext};

#[derive(Debug)]
pub enum NodeSlot {
    Occupied(LRUNode),
    Empty(PrevAndNext),
}

impl NodeSlot {
    pub fn new(query: String, prev: Option<usize>, next: Option<usize>) -> NodeSlot {
        let node = LRUNode::new(query, prev, next);
        NodeSlot::Occupied(node)
    }

    pub fn update_next(&mut self, index: Option<usize>) {
        if let NodeSlot::Occupied(node) = self {
            node.update_next(index);
        }
    }

    pub fn get_next_and_prev(&self) -> Option<PrevAndNext> {
        if let NodeSlot::Occupied(node) = self {
            let next_and_prev = node.get_next_and_prev();

            return Some(next_and_prev);
        }

        None
    }

    pub fn update_previous(&mut self, index: Option<usize>) {
        if let NodeSlot::Occupied(node) = self {
            node.update_previous(index)
        }
    }

    pub fn get_next(&self) -> Option<usize> {
        if let NodeSlot::Occupied(node) = self {
            return node.get_next();
        }

        None
    }

    pub fn get_prev(&self) -> Option<usize> {
        if let NodeSlot::Occupied(node) = self {
            return node.get_prev();
        }

        None
    }

    pub fn get_key(&self) -> Option<String> {
        if let NodeSlot::Occupied(node) = self {
            return Some(node.get_key());
        }
        None
    }

    pub fn make_empty(&mut self) {
        if let Some(next_prev) = self.get_next_and_prev() {
            *self = NodeSlot::Empty(next_prev);
        }
    }
    
    pub fn make_empty_occupied(&mut self, query: String) {
        if let NodeSlot::Empty(p_n) = self {
            let new_self = self::NodeSlot::new(query, p_n.get_prev(), p_n.get_next());
            *self = new_self;
        }
    }
}
