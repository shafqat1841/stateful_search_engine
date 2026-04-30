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