#[derive(Debug)]
pub struct PrevAndNext {
    pub prev: Option<usize>,
    pub next: Option<usize>,
}

impl PrevAndNext {
    pub fn new(prev: Option<usize>, next: Option<usize>) -> PrevAndNext {
        PrevAndNext { next, prev }
    }
}

#[derive(Debug)]
pub struct LRUNode {
    key: String,
    prev: Option<usize>,
    next: Option<usize>,
}

impl LRUNode {
    pub fn new(key: String, prev: Option<usize>, next: Option<usize>) -> LRUNode {
        LRUNode { key, prev, next }
    }

    pub fn get_key(&self) -> String {
        self.key.clone()
    }

    pub fn get_prev(&self) -> Option<usize> {
        let prev = self.prev;
        prev
    }

    pub fn get_next(&self) -> Option<usize> {
        let next = self.next;
        next
    }

    pub fn update_previous(&mut self, index: Option<usize>) {
        self.prev = index
    }

    pub fn update_next(&mut self, index: Option<usize>) {
        self.next = index
    }

    pub fn get_next_and_prev(&self) -> PrevAndNext {
        let prev = self.get_prev();
        let next = self.get_next();

        PrevAndNext::new(prev, next)
    }
}
