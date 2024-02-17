use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            value,
            next: None,
            prev: None,
        }))
    }
}

#[derive(Debug)]
pub struct DList<T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
}

impl<T> DList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|head| Ref::map(head.borrow(), |node| &node.value))
    }

    pub fn peek_front_mut(&self) -> Option<RefMut<T>> {
        self.head
            .as_ref()
            .map(|head| RefMut::map(head.borrow_mut(), |node| &mut node.value))
    }

    pub fn peek_back(&mut self) -> Option<Ref<T>> {
        self.tail
            .as_ref()
            .map(|tail| Ref::map(tail.borrow(), |node| &node.value))
    }

    pub fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
        self.tail
            .as_ref()
            .map(|tail| RefMut::map(tail.borrow_mut(), |node| &mut node.value))
    }

    pub fn push_front(&mut self, value: T) {
        let new_head = Node::new(value);

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }

        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            }

            self.len -= 1;
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().value
        })
    }

    pub fn push_back(&mut self, value: T) {
        let new_tail = Node::new(value);

        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().prev = Some(new_tail.clone());
                new_tail.borrow_mut().next = Some(old_tail);
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }

        self.len += 1;
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().next.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().prev.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }

            self.len -= 1;
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().value
        })
    }
}

impl<T> Drop for DList<T> {
    fn drop(&mut self) {
        let mut curr = self.head.take();

        while let Some(node) = curr {
            curr = node.borrow_mut().next.take();
        }
    }
}

pub struct IntoIter<T>(DList<T>);

impl<T> IntoIterator for DList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_pop_front() {
        let mut list = DList::new();
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        list.push_front(4);
        list.push_front(5);

        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn peek() {
        let mut list = DList::new();
        assert!(list.peek_front().is_none());

        list.push_front(1);
        list.push_front(2);
        assert_eq!(&*list.peek_front().unwrap(), &2);
    }

    #[test]
    fn push_pop_back() {
        let mut list = DList::new();
        assert_eq!(list.pop_back(), None);
        assert_eq!(list.len(), 0);

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));

        list.push_back(4);
        list.push_back(5);

        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_back(), Some(4));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
        assert_eq!(list.len(), 0);
    }
}
