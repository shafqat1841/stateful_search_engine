#[derive(Debug, Clone, Copy)]
pub struct PrevAndNext {
    prev: Option<usize>,
    next: Option<usize>,
}

impl PrevAndNext {
    pub fn new(prev: Option<usize>, next: Option<usize>) -> PrevAndNext {
        PrevAndNext { next, prev }
    }

    pub fn get_prev(&self) -> Option<usize> {
        self.prev
    }

    pub fn update_prev(&mut self, new_prev: Option<usize>) {
        self.prev = new_prev;
    }

    pub fn get_next(&self) -> Option<usize> {
        self.next
    }

    pub fn update_next(&mut self, new_next: Option<usize>) {
        self.next = new_next;
    }
}
