use std::rc::Rc;
use std::cell::RefCell;

type Link = Option<Rc<RefCell<Node>>>;

struct Node {
    value: u64,
    next: Vec<Link>,
}

pub struct SkipList {
    head: Link,
    tails: Vec<Link>,
    max_level: usize,
    len: usize,
}

impl SkipList {
    pub fn new() -> Self {
        todo!()
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

